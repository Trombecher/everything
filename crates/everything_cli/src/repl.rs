use std::{
    collections::HashMap,
    fs,
    io::{Write, stdin, stdout},
};

use everything::{LazyObject, ctx::EvaluationContext, ext::ObjectExt};
use everything_structures::{Object, Structure};
use everything_structures_ff::Parsable;

use crate::{names::NAMES, util::expand_vars};

fn variables_prelude(variables: &mut HashMap<Box<str>, Object>) {
    for (key, value) in NAMES.iter().cloned() {
        variables.insert(key.into(), value);
    }
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

        match expression.eval(knowledge, &mut EvaluationContext::default()) {
            LazyObject::Eager(object) => {
                println!("{object:?}")
            }
            LazyObject::LazySetValues(values) => {
                println!("Lazy set values:");

                for value in values {
                    println!("{value:?}")
                }
            }
        }

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
