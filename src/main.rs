use clap::{App, Arg, SubCommand};
use std::io;

use minipython::char_stream::CharStream;
use minipython::tokenizer::Tokenizer;
use minipython::parser::Parser;

fn main() -> io::Result<()> {
    let matches = App::new("minipython")
        .subcommands(vec![
            SubCommand::with_name("lc").arg(Arg::with_name("file").required(true)),
            SubCommand::with_name("apos").arg(Arg::with_name("file").required(true)),
            SubCommand::with_name("tokenize").arg(Arg::with_name("file").required(true)),
            SubCommand::with_name("parser").arg(Arg::with_name("file").required(true)),
        ])
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("lc") {
        CharStream::new(matches.value_of("file").unwrap())?.lc();
    } else if let Some(matches) = matches.subcommand_matches("apos") {
        CharStream::new(matches.value_of("file").unwrap())?.apos();
    } else if let Some(matches) = matches.subcommand_matches("tokenize") {
        Tokenizer::new(matches.value_of("file").unwrap())?.tokenize();
    } else if let Some(matches) = matches.subcommand_matches("parse") {
        Parser::new(matches.value_of("file").unwrap())?.parse();
    }

    Ok(())
}
