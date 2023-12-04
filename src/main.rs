mod codegen;
mod combinators;

use codegen::generate_code;
use combinators::attr_cmb::{attribute_tag, Attribute};
use combinators::state_cmb::{state_tag, State};

use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;

use notify::{EventKind, RecursiveMode, Watcher};

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{multispace0, space0, space1},
    IResult,
};

pub struct Widget {
    name: String,
    positional_attributes: Vec<Attribute>,
    named_attributes: Vec<Attribute>,
    states: Vec<State>,
}

fn dir_watcher(path: &Path) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();

    watcher.watch(path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                let u_event = event.unwrap();
                let event_kind = u_event.kind;
                match event_kind {
                    EventKind::Create(_) => {
                        let path_buf = u_event.paths[0].to_path_buf();
                        let extension = path_buf.extension();

                        if extension.is_some() && extension.unwrap() == "arrow" {
                            println!("File created: {:?}", u_event.paths);
                            process_file(&path_buf).unwrap();
                        }
                    }
                    EventKind::Modify(_) => {
                        let path_buf = u_event.paths[0].to_path_buf();
                        let extension = path_buf.extension();

                        if extension.is_some() && extension.unwrap() == "arrow" {
                            println!("File modified: {:?}", u_event.paths);
                            process_file(&path_buf).unwrap();
                        }
                    }
                    EventKind::Remove(_) => {
                        let path_buf = u_event.paths[0].to_path_buf();
                        let extension = path_buf.extension();

                        if extension.is_some() && extension.unwrap() == "arrow" {
                            println!("File removed: {:?}", u_event.paths);
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();
    let mut path = String::from(".");
    let mut watch = false;

    for arg in args {
        if arg == "--watch" {
            watch = true;
        } else {
            path = arg;
        }
    }

    let path = Path::new(&path);

    if watch {
        dir_watcher(path)?;
    } else {
        let file = Path::new(path).to_path_buf();

        process_file(&file)?;
    }

    Ok(())
}

fn process_file(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(file)?;
    let res = parse(&source);

    match res {
        Ok((_, code)) => {
            // create a file with the same name as the input file, but with a .dart extension
            let mut output_file_path = file.clone();
            output_file_path.set_extension("dart");

            let output_file_path = output_file_path.to_str().expect("Invalid path");

            std::fs::write(output_file_path, code)?;
            // run dart format in the output file

            let output = std::process::Command::new("dart")
                .arg("format")
                .arg(output_file_path)
                .output()?;

            if !output.status.success() {
                println!("Error: {:?}", output);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}

fn parse(source: &str) -> IResult<&str, String> {
    let (input, result) = program(source)?;

    Ok((input, result))
}

fn program(input: &str) -> IResult<&str, String> {
    let mut widget = Widget {
        name: String::from(""),
        positional_attributes: vec![],
        named_attributes: vec![],
        states: vec![],
    };

    let (input, _) = multispace0(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("widget")(input)?;
    let (input, _) = space1(input)?;

    let (input, widget_name) = take_until1(" ")(input)?;
    widget.name = String::from(widget_name);

    let (input, _) = space0(input)?;
    let (input, _) = tag("{")(input)?;

    let (input, _) = multispace0(input)?;

    let mut input = input;
    while !input.starts_with("}") {
        if input.starts_with("state") {
            let (remaining_input, state) = state_tag(input)?;
            widget.states.push(state);

            let (remaining, _) = multispace0(remaining_input)?;
            let (remaining, _) = space0(remaining)?;

            input = remaining;
            continue;
        }

        let (remaining_input, attribute) = attribute_tag(input)?;

        if attribute.is_positional {
            widget.positional_attributes.push(attribute);
        } else {
            widget.named_attributes.push(attribute);
        }

        let (remaining, _) = multispace0(remaining_input)?;
        let (remaining, _) = space0(remaining)?;

        input = remaining;
    }

    let (_, _) = tag("}")(input)?;

    let (_, code) = generate_code(widget).unwrap();

    Ok(("", code))
}
