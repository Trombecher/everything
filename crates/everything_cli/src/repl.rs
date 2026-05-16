use std::{
    collections::HashMap,
    fs,
    io::{Write, stdin, stdout},
};

use everything::{
    base::BASE,
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt},
};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;

use crate::util::expand_vars;

fn variables_prelude(variables: &mut HashMap<Box<str>, Object>) {
    variables.insert("BASE".into(), BASE.clone().into());

    variables.insert("ZERO".into(), Abstract::ZERO.into());
    variables.insert("SUCCESSOR_OF".into(), Abstract::SUCCESSOR_OF.into());
    variables.insert("PREDECESSOR_OF".into(), Abstract::PREDECESSOR_OF.into());
    variables.insert("LIST_ITEM".into(), Abstract::LIST_ITEM.into());
    variables.insert("LIST_TAIL".into(), Abstract::LIST_TAIL.into());
    variables.insert("CODE_POINT".into(), Abstract::CODE_POINT.into());
    variables.insert("BIT_0".into(), Abstract::BIT_0.into());
    variables.insert("BIT_1".into(), Abstract::BIT_1.into());
    variables.insert("BIT_SLOT_0".into(), Abstract::BIT_SLOT_0.into());
    variables.insert("BIT_SLOT_1".into(), Abstract::BIT_SLOT_1.into());
    variables.insert("BIT_SLOT_2".into(), Abstract::BIT_SLOT_2.into());
    variables.insert("BIT_SLOT_3".into(), Abstract::BIT_SLOT_3.into());
    variables.insert("BIT_SLOT_4".into(), Abstract::BIT_SLOT_4.into());
    variables.insert("BIT_SLOT_5".into(), Abstract::BIT_SLOT_5.into());
    variables.insert("BIT_SLOT_6".into(), Abstract::BIT_SLOT_6.into());
    variables.insert("BIT_SLOT_7".into(), Abstract::BIT_SLOT_7.into());

    variables.insert("CONTAINS".into(), Abstract::CONTAINS.into());
    variables.insert("AXIOMATIC".into(), Abstract::AXIOMATIC.into());
    variables.insert("COMPUTED".into(), Abstract::COMPUTED.into());
    variables.insert(
        "STATEMENT_SUBJECT".into(),
        Abstract::STATEMENT_SUBJECT.into(),
    );
    variables.insert("STATEMENT_TAG".into(), Abstract::STATEMENT_TAG.into());
    variables.insert("STATEMENT_VALUE".into(), Abstract::STATEMENT_VALUE.into());
    variables.insert("KNOWLEDGE".into(), Abstract::KNOWLEDGE.into());
    variables.insert("NODE_LITERAL".into(), Abstract::NODE_LITERAL.into());
    variables.insert("NODE_AND_LEFT".into(), Abstract::NODE_AND_LEFT.into());
    variables.insert("NODE_AND_RIGHT".into(), Abstract::NODE_AND_RIGHT.into());
    variables.insert("NODE_PARAMETER".into(), Abstract::NODE_PARAMETER.into());
    variables.insert("NODE_COUNT".into(), Abstract::NODE_COUNT.into());
    variables.insert("NODE_QUERY".into(), Abstract::NODE_QUERY.into());
    variables.insert("NODE_EQUAL_LEFT".into(), Abstract::NODE_EQUAL_LEFT.into());
    variables.insert("NODE_EQUAL_RIGHT".into(), Abstract::NODE_EQUAL_RIGHT.into());
    variables.insert("NODE_OR_LEFT".into(), Abstract::NODE_OR_LEFT.into());
    variables.insert("NODE_OR_RIGHT".into(), Abstract::NODE_OR_RIGHT.into());
    variables.insert("NODE_XOR_LEFT".into(), Abstract::NODE_XOR_LEFT.into());
    variables.insert("NODE_XOR_RIGHT".into(), Abstract::NODE_XOR_RIGHT.into());
    variables.insert("NODE_NOT".into(), Abstract::NODE_NOT.into());
    variables.insert(
        "NODE_FUNCTION_SELF".into(),
        Abstract::NODE_FUNCTION_SELF.into(),
    );
    variables.insert("NODE_ADD_LEFT".into(), Abstract::NODE_ADD_LEFT.into());
    variables.insert("NODE_ADD_RIGHT".into(), Abstract::NODE_ADD_RIGHT.into());
    variables.insert(
        "ARITHMETIC_OVERFLOW".into(),
        Abstract::ARITHMETIC_OVERFLOW.into(),
    );
    variables.insert("UNDEFINED".into(), Abstract::UNDEFINED.into());
}

const HELP_MESSAGE: &str = "
    ?                   prints this message
    help                prints this message
    exit                exits this repl
    set <var> [object]  sets the given variable to the given object;
                            if object is omitted, the var is deleted
    get <var>           prints the object in the variable
    load <var> <path>   loads and parses a structure from the path
    eval <object>       evaluates an object
    vars                prints all variables
";

type Variables = HashMap<Box<str>, Object>;

pub struct Repl {
    variables: Variables,
}

impl Default for Repl {
    fn default() -> Self {
        let mut variables = Variables::new();
        variables_prelude(&mut variables);

        Self { variables }
    }
}

impl Repl {
    fn set_command(&mut self, arguments: &str) -> Result<(), String> {
        let Some((name, raw_object_text)) = arguments.split_once(" ") else {
            self.variables.remove(arguments);
            return Ok(());
        };

        let object_text = expand_vars(raw_object_text, &self.variables)?;
        let object = Object::parse(&object_text).map_err(|error| format!("{error:?}"))?;

        self.variables.insert(name.into(), object);

        Ok(())
    }

    fn load_command(&mut self, arguments: &str) -> Result<(), String> {
        let (name, path) = arguments
            .split_once(" ")
            .ok_or_else(|| format!("invalid usage"))?;

        let file_content =
            fs::read_to_string(path).map_err(|error| format!("reading file: {error:?}"))?;
        let structure = Structure::parse(&file_content).map_err(|error| format!("{error:?}"))?;

        self.variables.insert(name.into(), structure.into());

        Ok(())
    }

    fn eval_command(&mut self, arguments: &str) -> Result<(), String> {
        let Some(Object::Structure(knowledge)) = self.variables.get("knowledge") else {
            return Err(format!("variable 'knowledge' needs to be knowledge"));
        };

        let replaced = expand_vars(arguments, &self.variables)?;
        let expression = Object::parse(&replaced).map_err(|error| format!("{error:?}"))?;

        let output = expression.eval(knowledge, &mut EvaluationContext::default());

        println!("Result: {:?}", output);

        Ok(())
    }

    pub fn main_loop(&mut self) {
        println!("welcome to the experimental REPL. type ? + enter for help");

        let mut line = String::new();

        loop {
            print!("> ");
            stdout().flush().unwrap();

            line.clear();
            stdin().read_line(&mut line).unwrap();

            let line = line.trim();

            let (command, arguments) = line.split_once(" ").unwrap_or((line, ""));

            let result = match command {
                "exit" => break,
                "set" => self.set_command(arguments),
                "load" => self.load_command(arguments),
                "vars" => {
                    let mut first = true;

                    for variable in self.variables.keys() {
                        if !first {
                            print!(", ");
                        }

                        print!("{variable}");

                        first = false;
                    }

                    println!();

                    Ok(())
                }
                "get" => {
                    if let Some(object) = self.variables.get(arguments) {
                        println!("{object:?}");

                        Ok(())
                    } else {
                        Err(format!("variable '{arguments}' does not exist"))
                    }
                }
                "?" | "help" => {
                    println!("{}", HELP_MESSAGE);

                    Ok(())
                }
                "eval" => self.eval_command(arguments),
                _ => Err(format!("unknown command '{command}'")),
            };

            if let Err(result) = result {
                eprintln!("error: {result}")
            }
        }
    }
}
