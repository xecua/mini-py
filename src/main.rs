use clap::{App, Arg, SubCommand};
use std::io;

use minipython::char_stream::CharStream;
use minipython::eval::evaluator::Evaluator;
use minipython::parser::Parser;
use minipython::tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let matches = App::new("minipython")
        .subcommands(vec![
            SubCommand::with_name("lc").about("count up lines"),
            SubCommand::with_name("apos").about("output 'a' position"),
            SubCommand::with_name("tokenize").about("run only tokenizer"),
            SubCommand::with_name("parse").about("output AST"),
        ])
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("lc") {
        CharStream::new(matches.value_of("file").unwrap())?.lc();
    } else if let Some(_) = matches.subcommand_matches("apos") {
        CharStream::new(matches.value_of("file").unwrap())?.apos();
    } else if let Some(_) = matches.subcommand_matches("tokenize") {
        Tokenizer::new(matches.value_of("file").unwrap())?.tokenize();
    } else if let Some(_) = matches.subcommand_matches("parse") {
        println!(
            "{:?}",
            Parser::new(matches.value_of("file").unwrap())?.parse()
        );
    } else {
        Evaluator::new().eval_file_input(matches.value_of("file").unwrap())?;
    }

    Ok(())
}
