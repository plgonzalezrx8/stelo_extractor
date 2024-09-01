use clap::{Arg, Command};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

mod models;
mod parser;
mod utils;

use crate::parser::parse_xml;

fn create_command() -> Command<'static> {
    Command::new("Glucose Data Extractor")
        .version("0.3.1")
        .author("Pedro Gonzalez")
        .about("Extracts glucose data from Apple Health XML export")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input XML file")
                .required_unless_present("current-directory"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output JSON file")
                .required_unless_present("current-directory"),
        )
        .arg(
            Arg::new("current-directory")
                .long("current-directory")
                .help("Use current directory for input (export.xml) and output (output.json)")
                .conflicts_with_all(&["input", "output"]),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Print help information"),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = create_command();
    let matches = match cmd.try_get_matches_from_mut(std::env::args_os()) {
        Ok(m) => m,
        Err(e) => {
            // Let clap handle the error, which includes printing help if needed
            e.exit();
        }
    };

    // At this point, we know we have valid matches and help wasn't requested
    // (because clap would have handled it already if it was)

    let (input, output) = if matches.is_present("current-directory") {
        println!("Using current directory for input and output.");
        println!(
            "This will look for 'export.xml' in the current directory and output to 'output.json'."
        );
        print!("Do you want to continue? (y/n): ");
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if response.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return Ok(());
        }

        (PathBuf::from("export.xml"), PathBuf::from("output.json"))
    } else {
        (
            PathBuf::from(matches.value_of("input").unwrap()),
            PathBuf::from(matches.value_of("output").unwrap()),
        )
    };

    let start_time = Instant::now();

    println!("Starting to process XML file: {:?}", input);

    let (entries, count, blood_glucose_count) = parse_xml(&input)?;

    println!("XML processing complete. Writing JSON...");

    let json = serde_json::json!({ "blood_glucose_entries": entries });
    std::fs::write(&output, serde_json::to_string_pretty(&json)?)?;

    let duration = start_time.elapsed();
    println!(
        "Conversion complete in {:.2} seconds.",
        duration.as_secs_f64()
    );
    println!("Total Records processed: {}", count);
    println!("Total Blood Glucose entries: {}", blood_glucose_count);
    println!("JSON file saved as {:?}", output);
    Ok(())
}
