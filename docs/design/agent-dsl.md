# Agent DSL

This document discusses the design of a small DSL that LLM agents use to read from and write to Everything. A Rust process converts between Everything objects and the DSL; the objects remain the single source of truth. The DSL is the payload of that conversion, not a second data model.

The desktop app (a separate project) is out of scope. The DSL is designed _agents first_: it does not have to be human readable, although adopting the notation that the documentation already uses incidentally makes it quite readable.

## Motivation

The whole computation system and all knowledge are built out of Everything objects: atoms, sets, and constructions. Writing code or knowledge as these nested structures is cumbersome and token-hungry, both for humans and for LLM agents. Since agents have a limited context and are trained on text, they need a compact, consistent textual surface.

## Goals

- Agents can read, write, and edit knowledge and code through the DSL.
- The language is small: its entire specification fits on about one page, suitable for a system prompt and for a fine-tuning corpus.
- The language is trainable: it reuses notation and concepts that base LLMs already know from pretraining (lambda calculus, `map`/`filter`, infix arithmetic, and the notation already used throughout the Everything docs).
- Round-trips between DSL text and objects are lossless and deterministic.

## Non-Goals

- Human-first ergonomics (the desktop app covers humans).
- Types, modules, imports, I/O, exceptions, pattern matching (see "Deliberate Omissions").
- Replacing the object model.

## Principles

### 1. Design the loop, not just the format

An agent does not just write code; it lives in a cycle: discover names, read items, write/edit, receive errors, repair, evaluate, read the result. The DSL is only the payload of that cycle. The protocol around it must be specified as carefully as the language itself, as one small surface (see "Tool Protocol").

The rule of thumb: errors, results, and reads come back _in the same language the agent writes_. Agents self-repair best when the error is phrased in the language they just used.

### 2. Two tiers: a bijective core and lossless sugar

The language is strictly layered:

- **Core tier**: one-to-one with objects. Atom (alias or `@<u128>`), `{...}` sets, juxtaposition `c v` constructions, node atoms referenced by alias. The core is bijective with objects by construction, so `parse(print(o)) == o` is a property test, not a hope.
- **Sugar tier**: pure desugaring. Decimal integers, text, lists, lambdas, infix operators, calls, property lookup. Each sugar has a single desugaring, and the printer re-recognizes the canonical pattern and re-sugars it.

This keeps the converter small and correct: `Node::new` / `into_object` is already the tier boundary; the parser and printer slot in next to it.

### 3. Adopt the documentation notation

The docs already write `David is born {date_time : 10234823948}`, `{successor_of : zero}`, and `subject |-> value |-> ...`. If the DSL _is_ that notation (formalized), then:

- the existing documentation is seed training data,
- the desktop app can render the exact same canonical text (shared printer crate),
- every agent trajectory log, error message, and example becomes more data in the same language.

Everything compounds into one language instead of three. This is the main argument against S-expressions or JSON, even though both parse more easily: they would fork the artifacts. (S-expressions remain the fallback if the juxtaposition decision in principle 6 turns out to be painful.)

### 4. Named binders, never exposed De Bruijn indices

Internally, parameter references and function self-references use depths (`node_parameter : <depth>`, `node_function_self : <depth>`), i.e. De Bruijn-style indices. LLMs are reliably bad at counting indices across nesting; one off-by-one silently changes meaning. Therefore:

- The surface syntax uses `λx. body` with a named parameter. Functions are unary anyway, which makes names trivially unambiguous.
- The **converter** computes depths; the model never sees or writes an index.
- The **printer** regenerates canonical names from position, never echoing whatever name an author used, so round-trips are stable and training pairs are consistent.
- Recursion names the function at the definition site: inside `def length = λl. ... length(l.list_tail) ...`, the inner `length` desugars to `node_function_self : 0`. Recursion by name is the most pretraining-anchored form of recursion there is.

### 5. Aliases are the namespace, and they live in the knowledge

Agents cannot memorize `u128` atom ids, so the DSL speaks aliases. The alias registry is part of the knowledge, not a side channel: an alias is just a statement (e.g. `david is name "david"`). Then resolution is derived from the knowledge the converter already has, renaming is a data edit, and aliases are shared by every agent and the desktop app. Printing falls back to `@<id>` for anonymous atoms, so round-trips never lose information.

Two policies prevent the worst agent foot-guns:

1. **No silent atom creation.** An unresolvable alias in a reference position is a positioned error with edit-distance suggestions. Creation is explicit: `new david` (or the `prop`/`ctor`/`def` sugars, which imply it). Otherwise typos silently fork the ontology, and the model assumption (well-founded bijection between things and objects) dies by a thousand typos.
2. **Unique aliases per knowledge**, with deterministic disambiguation (e.g. `person#2`) when a collision is introduced.

### 6. Juxtaposition is construction; calls are parenthesized

The object model's `a : b` means _construction_, but expression intuition says `f x` means _call_. Both readings must not coexist on the same syntax. The resolution adopted here keeps juxtaposition as construction and always parenthesizes calls:

- `c v` juxtaposition is a construction (right-associative, as in the object model). In expression position it evaluates both sides and constructs, which is already the evaluation semantics of constructions, so "data is code" stays uniform (it corresponds to `NewConstruction` semantics).
- Calls are always parenthesized: `f(x)`, curried as `f(x)(y)`. Builtin node forms use the same shape (`map(s, f)`, `values(o, c)`) or infix (`if c then a else b`).
- Parens directly following an expression are a call, **whitespace-insensitively**; parens elsewhere are grouping. Right-associativity covers the common grouped-value case (`f g x` ≡ `f (g x)`), and left-nesting is writable as `((a b) c)`.
- One postfix sugar corrects the biggest intuition trap: `p.prop` desugars to `unwrap(values({p}, prop))`. Agents will want to write `p.born` and mean "p's birth timestamp"; bare juxtaposition `p born` would hand them the construction object instead. The postfix form gives the intuition the right semantics, and it chains: `p.born.date_time`.

The grammar stays LL(1)-ish with one standard precedence ladder, no significant whitespace, and a line-oriented top level. Models handle that world well; they handle indentation-sensitive and context-sensitive grammars poorly.

### 7. Token economy: compress the frequent, do not APL-ize the rest

- **Literals are the biggest win by far.** Peano numerals are O(n) tokens per number. Decimal integers, `"text"`, `[a, b, c]` lists, and byte sugar are non-negotiable.
- Short node names (`map`, `filter`, `fold`, `unwrap`) instead of `node_call_callee`-style names, from one table that is the single source of truth for parser, printer, docs, and the grammar card.
- Infix for exactly the classics (`=`, `<`, `+`, `*`, `and`/`or`/`not`): maximal pretraining anchorage per token.
- **Stop there.** A symbol the base model has never seen used that way costs more in fine-tuning data than it saves in tokens. "A small pure functional language plus set literals" is the sweet spot; the model transfers lambda calculus, `map`/`filter`, infix arithmetic, and SQL-ish thinking for free.

### 8. Deliberate omissions

- **Comments** cannot survive a round-trip through objects, so there are none. Intent-annotations are knowledge: `length is note "computes list length"`. Documentation becomes data, which is very much in the spirit of Everything.
- No types, modules, imports, pattern matching, string interpolation, or exceptions (`undefined` already exists as a value).
- No I/O. The language is pure, so agent-written code is sandboxed by construction, which is a genuinely valuable property.
- No multiple spellings in the **printer**. The parser may accept mild variants (e.g. `x |-> e`, which the docs use), but output is always one canonical form. Canonical output enables self-verification (agent writes, tool prints back, mismatch is an error), meaningful diffs, and consistent fine-tuning targets.

### 9. Canonical printing

- Every document starts with the version marker `!ev1`.
- One statement per line, no wrapping. Models do not care about line length, only token count.
- Nodes print through the `Node` API in fixed field order (`if c then a else b`, not set-sorted order). Non-node sets print in the structural `Ord` order of objects (already implemented).
- Canonical integers (the sign rules from Common Models), minimal text escaping, canonical alias choice, `@<id>` fallback.
- Same object, same text, forever. This one property powers the verification loop, diff-based edits, and corpus consistency.

### 10. Versioning constitution

- The `!ev1` marker (one token) on line 1 of every document. Training corpora live forever; a mixed corpus of two dialects is poison. The marker lets multiple language versions be trained on and served cleanly.
- **Additive-only evolution**: new node types are new keywords; existing spellings never change meaning. This should be written down as a constitution now.

## Proposed Syntax

The following is a proposal, not a fixed decision; the open points below call out where alternatives remain. The reference grammar ultimately lives with the parser.

### Lexical

```
!ev1                                  version marker (first line)
alias      := [a-z][a-z0-9_]*         alias, resolved against the knowledge
atom       := "@" <u128>              raw atom (escape hatch, never loses info)
integer    := -?[0-9]+                arbitrary precision
text       := "..."                   with \" \\ \n \u{...} escapes
```

Keywords are reserved and may not be aliases: `new prop ctor def is λ if then else and or xor not in true false undefined`.

### Statements (one per line)

```
new david
prop  person   = λ s. true
ctor  born     = λ s v. s is person
ctor  child_of = λ s v. s is person and v is person
def   double   = λ x. x + x
david is person
```

Desugaring:

- `new a` creates a fresh atom (ULID-style id) and registers the alias.
- `prop a = f` is `new a` plus `a is property <f>`.
- `ctor a = f` is `new a` plus `a is property_constructor <f>`.
- `def a = f` is `new a` plus `a is node_function : <f>` — atoms become callable functions because node detection already queries the knowledge for the `node_function` property.
- `a is e` is the plain statement with `e`'s object.

### Expressions (sketch)

```
expr       := "λ" params "." expr            function; multiple params = curried
            | "if" expr "then" expr "else" expr
            | or_expr

or_expr    := and_expr { ("or" | "xor" | "|") and_expr }   -- or, xor, union
and_expr   := { "not" } cmp_expr { "and" cmp_expr }
cmp_expr   := add_expr { ("=" | "<" | "is" | "in") add_expr }
add_expr   := mul_expr { "+" mul_expr }
mul_expr   := app_expr { "*" app_expr }
app_expr   := postfix { postfix }             juxtaposition = construction, right-assoc
postfix    := primary { "(" expr ")" | "." alias }   call, property lookup
primary    := alias | atom | integer | text | "true" | "false" | "undefined"
            | "{" [ expr { "," expr } ] "}"   set
            | "[" [ expr { "," expr } ] "]"   list sugar
            | "'" expr                        literal / quote
            | "(" expr ")"                    grouping
```

### Node sugar table

This table must stay one-to-one with the `Node` enum. It is the spec.

| sugar                        | node                                        |
| ---------------------------- | ------------------------------------------- |
| `λx. b`                      | Function                                    |
| `'e`                         | Literal (needed when data matches a node pattern) |
| `f(x)`                       | Call                                        |
| `a b` (expression position)  | ≈ NewConstruction semantics (evaluate parts, construct) |
| `[ a, b ]`                   | NewComposite                                |
| `a and b` / `a or b` / `a xor b` / `not a` | And / Or / Xor / Not            |
| `a = b` / `a < b`            | Equal / Less                                |
| `a + b` / `a * b`            | Add / Multiply                              |
| `a \| b`                     | Union                                       |
| `a in s`                     | Contains                                    |
| `count(s)` / `unwrap(s)`     | Count / Unwrap                              |
| `properties_of(x)`           | PropertiesOf                                |
| `has_value(x)`               | HasValue                                    |
| `has_property(s, p)`         | HasProperty                                 |
| `map(s, f)`                  | Map                                         |
| `filter(s, f)`               | Filter                                      |
| `every(s, p)` / `any(s, p)`  | Every / Any                                 |
| `fold(s, f, i)`              | Fold                                        |
| `if c then a else b`         | If                                          |
| `atoms_with(p)`              | QueryAtomsWithProperty                      |
| `atoms_with_ctor(c)`         | QueryAtomsWithConstructor                   |
| `values(o, c)`               | QueryConstructionValues                     |
| `constructions(c)`           | QueryConstructions                          |
| `is_atom(x)` / `is_set(x)` / `is_construction(x)` | IsAtom / IsSet / IsConstruction |
| `ctor_of(c)` / `val_of(c)`   | ConstructionConstructor / ConstructionValue |
| `x.p`                        | `unwrap(values(x, p))`                      |
| `true` / `false` / `undefined` | atoms                                      |

Set-taking builtins (`count`, `unwrap`, `map`, `filter`, …) require a set. Use `properties_of(x)` to pass an object's properties as a set. `values(o, c)` is a query rather than a set operation: it accepts any object, resolving atoms through the knowledge and sets through their members.

### Samples

A knowledge transaction an agent might write:

```
!ev1

prop  person   = λ s. true
ctor  born     = λ s v. s is person
ctor  child_of = λ s v. s is person and v is person

new david
david is person
david is born {date_time : 1700000000}

new anna
anna is person
anna is born {date_time : 4700000000}
anna is child_of david

def adults = λ people.
    filter(people, λ p. p.born.date_time > 2000)

def length = λ l.
    if l = {} then 0
    else 1 + length(l.list_tail)
```

Note `λ s v. body` (curried unary functions, the docs' `subject |-> value |-> ...` shape) and `s is person` as an expression (see open point 2). `p.born` is the single `born` value of `p` (`{date_time : N}`), and `.date_time` unwraps the timestamp out of it.

The core tier underneath: the canonical object for `λx. x + 1`, printed when no sugar pattern matches. This is exactly the docs' notation:

```
{node_function : {node_add_first : {node_parameter : 0},
                  node_add_second : {successor_of : zero}}}
```

An error exchange (in-language, positioned, repairable):

```
> write:
  david is persn
error 8:10: unknown alias 'persn'
  8 | david is persn
  |          ^^^^^ did you mean 'person'?
```

## Tool Protocol

The agent-facing surface includes the tool protocol as well as the language. Machine framing is JSON envelopes with DSL text payloads; the DSL stays the legible core.

- `names(pattern)` — search the alias registry. Without this, agents hallucinate aliases constantly.
- `read(alias | id | range)` — fetch specific items, never "dump the DB".
- `write(document)` — transactional: all-or-nothing, validated, with positioned errors (line:col, offending fragment re-printed canonically, plus a suggestion).
- `eval(expr)` — with a **fuel/step budget and timeout**. The evaluator is a stack machine with unbounded recursion; an agent will eventually write an infinite loop. Budget-exceeded must be a clean error, not a hang.
- Truncated printing for huge results (`{a, b, ... +9_997 more}`) with a cursor to fetch more. Context is the scarcest resource.

## Training and Fine-tuning Pipeline

1. **Property-based round-trip tests first**: `parse ∘ print = id` on random objects, `print ∘ parse = canonicalize` on random texts. These double as corpus-quality guarantees.
2. **Unlimited synthetic data**: sample random `Node` trees and objects, print them, and get (text, object) pairs. Because the core tier is bijective, label correctness is automatic.
3. **Trajectory data over static data**: log real agent sessions including rejected writes and their repairs. Failed-then-fixed pairs are the most valuable fine-tuning examples that exist.
4. **Grammar card in every example** (instruction-tuning format): the ~one-page spec, generated from the same table that drives the parser. Single source of truth.
5. **Frozen vocabulary once training starts**, additive-only afterwards, `!ev1` markers throughout.

## Integration Notes

- New crate (`everything_dsl`?) providing `parse` and `print` beside `Node::new` / `into_object`. Tests live in separate `tests.rs` files under the module, per the repository conventions.
- `def name = ...` desugars to the statement `name is node_function : <object>`. Node detection (`raw.rs`) already queries the knowledge, so atoms become callable functions with zero evaluator changes. But for these statements to validate, the node atoms (at least `node_function`) must be registered as property constructors in the base knowledge — currently only `property`, `property_constructor`, `successor_of`, and `predecessor_of` are. Verify this early.
- `Contains` and `NewConstruction` are implemented, so the `in` and juxtaposition sugar can ship.

## Open Points

These decisions are open and should be settled before implementation:

1. **Juxtaposition = construction with `f(x)` calls, or an S-expression core?** The proposal above bets on the docs' notation with parenthesized calls. The fallback is a Lisp-style core (`(def length (fn x ...))`), which parses more simply but forks the notation across docs, app, and agents, and costs more tokens.
2. **`is` inside expressions** (needed for constraint functions like `child_of` above): desugar to calling the property's constraint function, or to membership in `atoms_with(p)`? Both agree for atom subjects; the exact desugaring must be pinned down.
3. **Declaration forms**: are `prop`/`ctor`/`def` the only way to declare, or do raw statements (`x is property ...`) remain first-class? Proposal: parser accepts both, printer emits only the sugar.
4. **Alias statement shape**: which property constructor registers names (`x is name "x"`?), how uniqueness is enforced, and how collisions are disambiguated (`person#2`).
5. **Anonymous atoms in print**: raw `@<id>`, or short deterministic derived names (e.g. `a1234`) to save tokens?
6. **`new` semantics within one document**: multiple `new`s with the same alias (error?), id generation strategy, and whether `new` can reference the just-created atom in the same transaction.
7. **Printer aggressiveness**: how eagerly the printer re-sugars (e.g. does it always fold `unwrap(values(x, p))` back into `x.p`?), and what the tie-breaking rules are when multiple sugar patterns match.
8. **`.` lookup on multi-valued properties**: `x.p` uses `unwrap`, so multiple values collapse to `undefined`. Is a `x.p*` (or similar) form wanted for the whole value set?
9. **Fuel/step budget size** and the granularity of budget-exceeded error reporting.

## Verdict

The DSL should be a two-tier language: a bijective core that mirrors the documentation's object notation, plus a small, closed set of desugaring sugars chosen for pretraining anchorage and token economy. Binders are named and depths are never exposed. Aliases are knowledge, creation is explicit, printing is canonical, and the language is versioned additively. Around it sits an equally small tool protocol with positioned errors, fuel budgets, and name discovery — because the loop, not the format, is what agents actually operate.
