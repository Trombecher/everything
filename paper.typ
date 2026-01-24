#set enum(numbering: "(1)")

= The Everything Data Model

== Abstract

In this paper we outline the model humans use to describe the world and formalize it. This will enable us to store raw knowledge in an unambiguous, defined format.

== Things

Humans think in things. Every#underline[thing] we can think of is a thing. There is a set of things. The things themselves are just abstract; what is important is the semantics, the meaning of them. In everyday conversations, we as humans talk about things. And you can only talk about a thing if you can identify it. And if you cannot talk about a thing because you cannot identify it then it is not a thing because we could not think of it. Therefore there can only exist so many things as identifiers. In this paper we will use _object_ as a synonym for thing.

=== Identifying Things

The straightforward way for us humans to talk about a thing are #underline[names]. For example, we identify people by their names, places, activities, etc. The thing is (pun intended) that names can be ambiguous.

A person $R$ growing up with banks as in river banks may ought to uniquely and unambiguously identify the concept of river banks with the word "bank". A sales person $S$ in the city may use the word "bank" to uniquely and unambiguously identify the bank as in where you can store money. A conversation of banks between may lead to a common but interesting phenomenon. $R$ might say "Banks are interesting!" and $S$ would agree, as $S$ could understand how one might find banks (as in where you deposit money) interesting.

The interesting point is where $S$ would say: "Yeah, they store so much money". $R$: "Money? At the river?". And this is the tipping point. $R$ realizes that the property of banks described by $S$ is inapplicable to the object referenced by $R$. Both parties realize they have apparently been talking about different objects. They now have to adjust their object name / identifier because they have been made aware of the existence of a another similarly named object.

Now, $S$: "Yeah, you know? The banks that store money." $R$: "Oh there banks are that store money? I thought banks were the sides of a river..." $S$: "Well, these are called banks." Now the names have changed to "river bank" and "money bank".

=== Semantic Meaning

Identifiers -- like words -- are nice because they partly describe what the thing is. This makes it easy for us humans to remember them. However, this may lead to problems if the meaning ever changes.

Consider the example of two Michael Jacksons. To distinguish them, one may be called "the Michael Jackson that is at Remix" and the other one "Michael Jackson, the black one". But Michael Jackson, the musician, transitioned into white. The person stayed the same but he is not black anymore.

Meanings change, so names are not a good fit for identifiers. Only if we decouple the semantics from the id, there is nothing to misinterpret. Numbers, UUIDs, or ULIDs are a good fit.

=== Data

When we as humans talk about numbers or other forms of data, we usually do not reference them via a name but just write down the number we mean. There is _basically_ no ambiguity when talking about numbers but more so the interpretation of them. $42$ and $42$ refer to the same object, the number $42$. This is a different kind of object reference that is identified by itself.

== Relating Things

...

== Mathematical Description

Upfront there is a distinction to make between the theoretical model -- the _Everything data model (EDM)_ -- and the implementation -- the _Everything database_.

The EDM works with objects, states facts, and places them in relation to each other. To address and talk about the objects, the EDM uses a domain of object references / identifiers (ids). There are two distinct types of object ids: _abstract object ids (AOIs)_ with the set $O_A$ and _data object ids (DOIs)_ defined by the set $O_D$. These sets must be mutually exclusive: $O_A inter O_B = emptyset$. The set of all object ids is: $O := O_A union O_B$. $O$ is also referred to as the _set of all objects_; elements being called _objects_ or _object ids_.

=== Abstract Object Ids

An abstract object id is an opaque object reference that unambiguously identifies the object referenced whose meaning in the model is in no correlation with the id. Abstract object ids therefore do not carry any semantic information about the object and exist solely to identify objects.

=== Data Object Ids

A data object id unambiguously references the object that is the data itself. The "data" is just an integer. Therefore, $O_D$ is the set of all data values the model can handle.

== Associations

The EDM's primitive is not the object id but the connections between the objects. An _association_ is a three-tuple of object ids. Conceptually, associations connect objects with the following semantics:

- The first tuple entry is nicknamed _target_. It represents the object in question, the object stated about in the association.
- The second tuple entry is nicknamed _tag_. It represents the property of the target stated about in this association.
- The third (and final) tuple entry is nicknamed _value_. It represents the value the property has on the target.

The set of all associations is defined as follows: $A := O times O times O$. $(a, b, c)$ is an association iff $(a, b, c) in A$.

== Builtin Objects

The EDM is a self-restricting model. This will become apparent when defining the database in the following section. But for that it needs some builtin anchors such that it can interface with the implementation of the database which does the actual validation.

Therefore, we define the following set of builtin objects which are referenced by these ids. The $M$ stands for "meta" but does not have any role beyond that.

$
  M_"tag", M_"unique", M_"inferred", M_"requires", M_"requires.not", M_"requires.or", M_"requires.or.not" :in O_A
$

All these symbols reference different objects. Their behavior is explained in the constraints section.

== Databases

A database (also called knowledge base) $D$ is a finite set of associations ($D subset.eq A and |D| != infinity$) that satisfies constraints (1) to (9).

=== General Definitions

- A _value_ is an object. The alias shall denote that the object in question is used as the third member in an association.
- An object $t$ is a _tag_ iff $exists v in O : (t, M_"tag", v) in D$.
- An object $a$ is _tagged_ with an object $b$ and a value $c$ iff $(a, b, c) in D$.
- An association $(o, t, v)$ _exists_ (in a database $D$) iff $(o, t, v) in D$.
- $v in O$ is the computed value of $f in O$ and $o in O$ iff $"Compute"(f, o) = v$.
- A tag $t$ is _applicable_ to $o in O$ iff $"Match"(o, t)$ is true.
- An association _conceptually exists_ if it exists or virtually exists. Formally:

  $
    (o, t, v) in^~ D :<==> (o, t, v) in D or (o, t, v) in^therefore D
  $

$"Match"(o, t)$ and $"Compute"(f, v)$ will be defined later.

=== Deducted Associations

For a given database $D$, the _set of all deducted associations_ $D^therefore$ is defined as:

$
  D^therefore := {(o, t, v) in A | "Match"(o, t) and (exists f in O : (t, M_"inferred", f) in D and v in "Compute"(f, o))}
$

=== Conceptual Associations

For a given database $D$, the _set of all conceptual associations_ $D^diamond.small$ is defined as:

$
  D^diamond.small := D union D^therefore
$


With just a few steps, it can be shown that association in $D$ are not deducted.

Let $o,t,v in O$.

$
  & (o, t, v) in D and (o, t, v) in D^therefore \
  ==>^("Def." D^therefore) & (o, t, v) in D and (exists f in O : (t, M_"inferred", f) in D and "Match"(o, t) and "Compute"(f, o) = v) \
  ==>^((6) "alt") & (exists.not f in O : (t, M_"inferred", f) in D) \
  & and (exists f in O : (t, M_"inferred", f) in D and "Match"(o, t) and "Compute"(f, o) = v) \
  ==> & "false"
$

$
  therefore D inter D^therefore = emptyset
$

=== Constraints

1. Every tag, used in an association with a value $v$, must be tagged with $M_"tag"$ and a value $x$, $t$ must be applicable to $o$, and $v$ must be tagged with $x$. Here is the formal constraint:

  $
    forall (o, t, v) in D : "Match"(o, t) and (exists x,y in O : (t, M_"tag", x) in D and (v, x, y) in D^diamond.small)
  $

  The motivation behind this constraint is that you cannot arbitrarily tag objects with other objects. You have to declare the tag object as a tag. By doing this, you also constraint the objects that can be used as values in the association by defining a "type". This type is a tag and only objects can be used as values in this association that are tagged with this type.

2. Tags tagged with $M_"unique"$ enforce uniqueness on the value of associations they are involved in as a tag for a given object. Formally:

  $
    forall t in O : ((exists u in O : (t, M_"unique", u) in D) \
      ==> (forall x, y in O : ((o, t, x) in D and (o, t, y) in D ==> x = y)))
  $

  Movivation: sometimes you want a tag to be applicable maximum once per object.

3. $M_"tag"$ is unique. There can only be one type describing the values of a tag. Formally:

  $
    exists v in O : (M_"tag", M_"unique", v) in D
  $

  The motivation behind this constraint is that if you could tag a tag multiple times with $M_"tag"$, there would be multiple tag objects for a value to validate against. The model would need to define the validation behaviour for this case.

  But this case can be reduced to only one association: one can simply define a new type that describes the intended behavior (intersection or union) the user intended with the two or more associations by connecting expressions via the builtin constraints. Example:

  $
    {"Type1", "Type2", "Value1", "Value2", "Value3", "Type:0", "A"} subset.eq O \
    \
    E := {("Type:0", M_"tag", "Type:0"), (0, "Type:0", 0), ("Value1", "Type1", 0), \
      ("Value2", "Type1", 0), ("Value2", "Type2", 0), ("Value3", "Type2", 0), \
      ("A", M_"tag", "Type1"), ("A", M_"tag", "Type2")}
  $

  Here we define three values and two types. $"Type1"$ includes values 1 and 2 and $"Type2"$ includes value 2 and 3. Now what should $v in O$ in $(o, "A", v) in D$ be constraint to? $"Type1"$ or $"Type2"$, or perhaps both? By making $M_"tag"$ unique we force the user to think about what $v$ should be explicitly.

4. The tag $M_"constraint"$

5. $
    exists.not v in O : (M_"tag", M_"inferred", v) in D
  $

6. The tag $M_"inferred"$ declares a tag as inferred. Inferred tags cannot be used explicitly in associations. However, virtually they compute the association value if the target matches the constraint. Formally:

  $
    forall t in O : ((exists f in O : (t, M_"inferred", f) in D) ==> (exists.not o,v in O : (o, t, v) in D))
  $

  This implies (by contraposition of the implication in parentheses) that if there is an association in $D$, the tag of that association is not inferred. Formally ((6) alt):

  $
    forall t in O : ((exists o,v in O : (o, t, v) in D) ==> (exists.not f in O : (t, M_"inferred", f) in D))
  $

=== Match And Applicability

The motivation behind $"Match"(o, t)$ is to determine if tag $t$ is applicable to the object $o$. $t$ is applicable to $o$ if the $t$'s constraint matches the object.

- To make $t$ require that $o$ has a different tag $t'$, do $(t, M_"requires", t') in D$. Multiple tags can be required.
- To make $t$ require that $o$ does #underline[not] have $t'$, do $(t, M_"requires.not", t') in D$.
- To make $t$ require that $o$ has at least one of some tags $t_1, ..., t_n$, do $forall 1 <= i <= n : (t, M_"requires.or", t_i) in D$. However, if there are no $M_"requires.or"$s, then this property should be ignored.
- To make $t$ require that $o$ has not every one of tags $t_1, ..., t_n$, do: $forall 1 <= i <= n : (t, M_"requires.or.not", t_i) in D$.

These properties formalize into this definition:

$
  "Match"(o, t) :<==> & and.big_(t' in O \ (t, M_"requires", t') in D) (exists v in O : (o, t', v) in D^diamond.small) \
  & and and.big_(t' in O \ (t, M_"requires.not", t') in D) (exists.not v in O : (o, t', v) in D^diamond.small) \
  & and (or.big_(t' in O \ (t, M_"requires.or", t') in D) (exists v in O : (o, t', v) in D^diamond.small) or (exists.not t' in O : (t, M_"requires.or", t') in D)) \
  & and (or.big_(t' in O \ (t, M_"requires.or.not", t') in D) (exists.not v in O : (o, t', v) in D^diamond.small) or (exists.not t' in O : (t, M_"requires.or.not", t') in D)) \
$

=== Consequences

From constraint (3) follows that there are no empty databases.

=== Minimal Database

$
  {(M_"tag", M_"tag", M_"tag"), (M_"tag", M_"unique", 0), \
    (M_"unique", M_"tag", M_"object"),
    (M_"unique", M_"requires", M_"tag"),
    (M_"unique", M_"requires.not", M_"inferred"), \
    (M_"object", M_"tag", M_"object"), (M_"object", M_"inferred", 0), \
    (M_"inferred", M_"tag", M_"object"),
    (M_"inferred" M_"requires", M_"tag"), \
    (M_"requires", M_"tag", M_"tag"),(M_"requires", M_"requires", M_"tag"), \
    (M_"requires.not", M_"tag", M_"tag"),(M_"requires.not", M_"requires", M_"tag"), \
    (M_"requires.or", M_"tag", M_"tag"),(M_"requires.or", M_"requires", M_"tag"), \
    (M_"requires.or.not", M_"tag", M_"tag"),(M_"requires.or.not", M_"requires", M_"tag"), }
$
