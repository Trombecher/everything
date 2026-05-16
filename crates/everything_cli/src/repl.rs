use std::{
    collections::HashMap,
    fs,
    io::{Write, stdin, stdout},
};

use everything::{
    ctx::EvaluationContext,
    ext::{AbstractExt, ObjectExt},
};
use everything_structures::{Abstract, Object, Structure};
use everything_structures_ff::Parsable;

use crate::util::{expand_vars, handle_parse_error};

fn variables_prelude(variables: &mut HashMap<Box<str>, Object>) {
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

pub fn repl_main() {
    println!("welcome to the experimental REPL. type ? + enter for help");

    let mut variables = HashMap::<Box<str>, Object>::new();
    variables_prelude(&mut variables);

    let mut line = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        line.clear();
        stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        let (command, arguments) = line.split_once(" ").unwrap_or((line, ""));

        match command {
            "exit" => break,
            "set" => {
                if let Some((name, object_source)) = arguments.split_once(" ") {
                    let object_source = expand_vars(object_source, &variables);

                    // TODO: not hard error
                    let object = Object::parse(&object_source).unwrap();

                    variables.insert(name.into(), object);
                } else {
                    println!("invalid usage")
                }
            }
            "load" => {
                if let Some((name, path)) = arguments.split_once(" ") {
                    let file_content = fs::read_to_string(path).unwrap();
                    let structure = Structure::parse(&file_content)
                        .unwrap_or_else(|error| handle_parse_error(&file_content, &error));

                    variables.insert(name.into(), structure.into());
                } else {
                    println!("invalid usage")
                }
            }
            "get" => {
                if let Some(object) = variables.get(arguments) {
                    println!("{object:?}");
                } else {
                    println!("This variable does not exist")
                }
            }
            "?" => {
                println!(
                    "exit - exits REPL\n? - prints this message\neval <EXPR> - evaluate this expression"
                );
            }
            "eval" => {
                if let Some(Object::Structure(knowledge)) = variables.get("knowledge") {
                    let replaced = expand_vars(arguments, &variables);

                    // TODO: make this not hard error.
                    let expression = Object::parse(&replaced).unwrap();

                    let output = expression.eval(knowledge, &mut EvaluationContext::default());

                    println!("Result: {:?}", output);
                } else {
                    println!("please set knowledge to some structure")
                }
            }
            _ => {
                println!("Unknown command {command}");
            }
        }
    }
}
