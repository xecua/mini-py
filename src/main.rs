use std::io;
use clap::{
    App,
    Arg,
    SubCommand
};

use minipython::stream::CharStream;

fn main() -> io::Result<()> {
    let matches = App::new("minipython")
                    .subcommands(vec![
                        SubCommand::with_name("lc")
                            .arg(Arg::with_name("file")
                                .required(true)),
                        SubCommand::with_name("apos")
                            .arg(Arg::with_name("file")
                                .required(true)),
                    ])
                .get_matches()
    ;

    if let Some(matches) = matches.subcommand_matches("lc") {
        CharStream::new(matches.value_of("file").unwrap())?.lc();
    } else if let Some(matches) = matches.subcommand_matches("apos") {
        CharStream::new(matches.value_of("file").unwrap())?.apos();
    }

    Ok(())
}
