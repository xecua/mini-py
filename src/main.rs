use clap::{App, Arg, SubCommand};
use std::io;

use minipython::stream::CharStream;
use minipython::tokenizer::Tokenizer;

fn main() -> io::Result<()> {
    let matches = App::new("minipython")
        .subcommands(vec![
            SubCommand::with_name("lc").arg(Arg::with_name("file").required(true)),
            SubCommand::with_name("apos").arg(Arg::with_name("file").required(true)),
            SubCommand::with_name("tokenize").arg(Arg::with_name("file").required(true)),
        ])
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("lc") {
        CharStream::new(matches.value_of("file").unwrap())?.lc();
    } else if let Some(matches) = matches.subcommand_matches("apos") {
        CharStream::new(matches.value_of("file").unwrap())?.apos();
    } else if let Some(matches) = matches.subcommand_matches("tokenize") {
        use minipython::token::Token::EOF;
        let mut t = Tokenizer::new(matches.value_of("file").unwrap())?;
        loop {
            t.next_token();
            println!("{:?}", t.get_current_token());
            if t.get_current_token() == EOF {
                break;
            }
        }
    }

    Ok(())
}
