// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

use std::{
    io::{IsTerminal, Read},
    process,
};

use ason::ast::{
    parser::parse_from_str,
    printer::{print_to_string, print_to_writer},
};
use clap::Parser;

/// ASON Query is a powerful tool for querying, manipulating and generating ASON data.
///
/// Resources:
///
/// - The ASON document:
///   https://hemashushu.github.io/works/ason/
///
/// - The ASON Query Language:
///   https://hemashushu.github.io/works/ason-query/
///
/// - The ASON Query Examples:
///   https://github.com/hemashushu/ason-query/examples
///
#[derive(Parser, Debug)]
#[command(name = "aq")]
#[command(version, about)]
// #[command()]
struct AqArgs {
    /*
    To specify the flags for an argument, you can use #[arg(short = 'n')] and/or #[arg(long = "name")]
    attributes on a field. When no value is given (e.g. #[arg(short)]),
    the flag is inferred from the field’s name.
    https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_2/index.html#options
     */
    /// Specify the output file
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,

    /* the query expression will be omitted if the query file is specified */
    /// Specify the query file
    #[arg(short, long, value_name = "QUERY_FILE")]
    query: Option<String>,

    /*
    a default value can be specified:
    https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_2/index.html#defaults
     */
    /// The query expression
    query_expression: Option<String>,

    /// Specify the input file(s)
    input_files: Vec<String>,
}

fn main() {
    // Usage:
    //   aq [options] <query expression> <input file(s)>
    //   aq [options] -o <output file> <query expression> <input file(s)>
    //   aq [options] -o <output file> -q <query file> <input file(s)>
    //
    // Command options:
    //   -o, --output=FILE      specify the output file
    //   -q, --query=FILE       specify the query file

    // Run with Cargo
    // --------------
    //
    // `$ cargo run -- <args and options>`
    //
    // e.g.
    // `cargo run -- . examples/01-primitive.ason`
    //
    // Run standalone
    // --------------
    //
    // `aq . examples/01-primitive.ason`
    //
    // Examples
    // --------
    //
    // - Read text from STDIN and apply the query, and then write the result to STDOUT.
    //
    //   $ echo '{id: 123, name: "John"} | aq '.id'
    //
    //   $ cat << EOF | aq '.id'
    //   > {
    //   >   id: 123
    //   >   name: "John"
    //   > }
    //   > EOF
    //
    // - Read text from a specify file and apply the query, and then write the result to STDOUT.
    //
    //   $ aq '.orders[].id' sales.ason
    //   $ cat sales.ason | aq '.orders[].id'
    //   $ aq '.orders[].id' < sales.ason
    //
    // - Read text from STDIN and apply the query, and then write the result to the specify file.
    //
    //   $ echo '[11, 13, 17, 19]' | aq -o numbers.ason '.filter(. > 13)'
    //   $ echo '[11, 13, 17, 19]' | aq '.filter(. > 13)' > numbers.ason
    //
    // - Read text from STDIN, read the query expression from the specify file and apply
    //   the query, and then write the result to STDOUT.
    //
    //   $ echo '[11, 13, 17, 19]' | aq -q example.aql

    // let mut args = std::env::args();
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    // let title = format!("ASON Query {}", env!("CARGO_PKG_VERSION"));
    // println!("{}", title);
    // process::exit(1);

    let aq_args = AqArgs::parse();

    // Note:
    //
    // - The STDIN will be omitted if INPUT_FILES is specified.
    // - The QUERY_EXPRESSION will be omitted if QUERY_FILE is specified.
    // - The STDOUT will be omitted if OUTPUT_FILE is specified.

    let mut texts = vec![];

    if !aq_args.input_files.is_empty() {
        // if let Some(f) = aq_args.inputs {

        println!("{:?}", aq_args.input_files);

        for f in aq_args.input_files {
            // text from input file
            match std::fs::read_to_string(&f) {
                Ok(s) => {
                    texts.push(s);
                }
                Err(e) => {
                    eprintln!("Fail to read the specified input file: \"{}\".", &f);
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
    } else {
        // text from STDIN
        let mut i = std::io::stdin().lock();
        if i.is_terminal() && aq_args.query_expression.is_none() {
            eprintln!("Usage: aq [OPTIONS] [QUERY_EXPRESSION]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
            process::exit(1);
        }

        let mut buf = String::new();
        match i.read_to_string(&mut buf) {
            Ok(_) => {
                texts.push(buf);
            }
            Err(e) => {
                eprintln!("Fail to read the input text from STDIN.");
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    };

    let mut nodes = vec![];

    for text in texts {
        match parse_from_str(&text) {
            Ok(n) => {
                nodes.push(n);
            }
            Err(e) => {
                eprintln!("{}", e.with_source(&text));
                process::exit(1);
            }
        }
    }

    let root = if nodes.len() == 1 {
        nodes.remove(0)
    } else {
        ason::ast::AsonNode::Tuple(nodes)
    };

    if let Some(f) = aq_args.output {
        match std::fs::write(&f, print_to_string(&root)) {
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
        match print_to_writer(&mut w, &root) {
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
