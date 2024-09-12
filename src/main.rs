// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

use std::{
    io::{IsTerminal, Read},
    process,
};

// use argh::FromArgs;
use ason::{parse_from_str, print_to_string, print_to_writer};
use clap::Parser;

// #[derive(FromArgs, Debug)]

/// ASON Query is a powerful tool for querying, manipulating and generating ASON data.
///
/// Examples:
///
/// - Read text from STDIN and apply the query, and then write the result to STDOUT.
///
///   $ echo '{id: 123, name: "John"} | aq '.id'
///
/// - Read text from a specify file and apply the query, and then write the result to STDOUT.
///
///   $ aq -i sales.ason '.orders[].id'
///
///   $ cat sales.ason | aq '.orders[].id'
///
///   $ aq '.orders[].id' < sales.ason
///
/// - Read text from STDIN and apply the query, and then write the result to the specify file.
///
///   $ echo '[11, 13, 17, 19]' | aq -o numbers.ason '.[].filter(. > 13)'
///
///   $ echo '[11, 13, 17, 19]' | aq '.[].filter(. > 13)' > numbers.ason
///
/// - Read text from STDIN, read the query string from the specify file and apply
///   the query, and then write the result to STDOUT.
///
///   $ echo '[11, 13, 17, 19]' | aq -q example.aql
///
/// Note:
///
/// - The STDIN will be omitted if INPUT_FILE is specified.
///
/// - The INPUT_FILE will be omitted if INPUT_TEXT is specified.
///
/// Resources:
///
/// - The ASON document:
///   https://hemashushu.github.io/works/ason/
///
/// - The ASON Query Language:
///   https://hemashushu.github.io/works/ason-query/
///
#[derive(Parser, Debug)]
#[command(name = "aq")]
#[command(version, about)]
struct AqArgs {
    // #[argh(option, short='i')]

    /*
    To specify the flags for an argument, you can use #[arg(short = 'n')] and/or #[arg(long = "name")]
    attributes on a field. When no value is given (e.g. #[arg(short)]),
    the flag is inferred from the field’s name.
    https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_2/index.html#options
     */
    /// Specify the input file
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,

    // #[argh(option, short='o')]
    /// Specify the output file
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,

    // #[argh(option, short='q')]
    /// Specify the query file
    #[arg(short, long, value_name = "QUERY_FILE")]
    query: Option<String>,

    // #[argh(option, short='t')]
    /// Specify the input text
    #[arg(short, long, value_name = "INPUT_TEXT")]
    text: Option<String>,

    // #[argh(positional, default = "String::from(\".\")")]
    /// The query string
    // #[arg(default_value_t = String::from("."))]
    query_string: Option<String>,
}

fn main() {
    // Usage:  aq [options] <query string>
    //         aq [options] -i <input file> <query string>
    //         aq [options] -i <input file> -o <output file> <query string>
    //         aq [options] -i <input file> -o <output file> -q <query file>
    //         aq [options] -t <input text> <query string>
    //
    // Command options:
    //   -i, --input=FILE       specify the input file
    //   -o, --output=FILE      specify the output file
    //   -q, --query=FILE       specify the query file
    //   -t, --text=TEXT        specify the input text

    // run with Cargo
    // --------------
    //
    // `$ cargo run -- <args and options>`
    //
    // e.g.
    // `cargo run -- -i examples/01-primitive.ason .`
    // `cargo run -- -i examples/06-error.ason`
    //
    // run standalone
    // --------------
    //
    // `aq -i examples/01-primitive.ason .`
    // `aq -i examples/06-error.ason`
    //
    // read text from STDIN
    // ---------------
    // `$ echo '{id: 123}' | aq '.id'`
    //
    // ```sh
    // cat << EOF | aq '.id'
    // > {
    // >   id: 123
    // >   name: "John"
    // > }
    // > EOF
    //```

    // let mut args = std::env::args();
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    // let title = format!("ASON Query {}", env!("CARGO_PKG_VERSION"));
    // println!("{}", title);
    // process::exit(1);

    // let aq_args:AqArgs = argh::from_env();

    let aq_args = AqArgs::parse();

    let source = if let Some(t) = aq_args.text {
        // text from arg
        t
    } else if let Some(f) = aq_args.input {
        // text from input file
        match std::fs::read_to_string(&f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Fail to read the specified input file: \"{}\".", &f);
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    } else {
        // text from STDIN
        let mut i = std::io::stdin().lock();
        if i.is_terminal() && aq_args.query_string.is_none() {
            eprintln!("Usage: aq [OPTIONS] [QUERY_STRING]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
            process::exit(1);
        }

        let mut buf = String::new();
        match i.read_to_string(&mut buf) {
            Ok(_) => buf,
            Err(e) => {
                eprintln!("Fail to read the input text from STDIN.");
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    };

    let node = match parse_from_str(&source) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}", e.with_source(&source));
            process::exit(1);
        }
    };

    if let Some(f) = aq_args.output {
        match std::fs::write(&f, print_to_string(&node)) {
            Err(e) => {
                eprintln!("Fail to write to the output file: \"{}\".", f);
                eprintln!("{}", e);
                process::exit(1);
            }
            Ok(_) => {
                // ok
            }
        }
    } else {
        let mut w = std::io::stdout().lock();
        match print_to_writer(&mut w, &node) {
            Err(e) => {
                eprintln!("Fail to write to the STDOUT.");
                eprintln!("{}", e);
                process::exit(1);
            }
            Ok(_) => {
                //
            }
        }
    }
}
