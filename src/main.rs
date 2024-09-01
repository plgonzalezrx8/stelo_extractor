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
        .version("1.0")
        .author("Your Name")
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
    let cmd = create_command();
    let matches = match cmd.try_get_matches() {
        Ok(m) => m,
        Err(e) => {
            let _ = cmd.print_help();
            println!("\nError: {}", e);
            return Ok(());
        }
    };

    // If we reach here, it means we have valid matches
    // Check if help was requested
    if matches.is_present("help") {
        cmd.print_help()?;
        return Ok(());
    }

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
