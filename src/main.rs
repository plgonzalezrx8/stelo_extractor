use chrono::DateTime;
use chrono_tz::America::New_York;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct BloodGlucoseEntry {
    #[serde(rename = "sourceName")]
    source_name: String,
    #[serde(rename = "creationDate")]
    creation_date: String,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
    time: String,
    value: String,
    unit: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    println!("Starting to process XML file...");

    let file = File::open("./export.xml")?;
    let file_size = file.metadata()?.len();
    let buf_reader = BufReader::new(file);

    let mut reader = Reader::from_reader(buf_reader);
    reader.trim_text(true);

    let mut entries = Vec::new();
    let mut buf = Vec::new();
    let mut count = 0;
    let mut blood_glucose_count = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) if e.name().as_ref() == b"Record" => {
                count += 1;

                if let Some(ty) = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .find(|attr| attr.key.as_ref() == b"type")
                {
                    if ty.unescape_value()?.into_owned() == "HKQuantityTypeIdentifierBloodGlucose" {
                        blood_glucose_count += 1;
                        let mut entry = BloodGlucoseEntry {
                            source_name: String::new(),
                            creation_date: String::new(),
                            start_date: String::new(),
                            end_date: String::new(),
                            time: String::new(),
                            value: String::new(),
                            unit: String::new(),
                        };

                        for attr in e.attributes().filter_map(|a| a.ok()) {
                            match attr.key.as_ref() {
                                b"sourceName" => {
                                    entry.source_name = attr.unescape_value()?.into_owned()
                                }
                                b"creationDate" => {
                                    entry.creation_date = attr.unescape_value()?.into_owned()
                                }
                                b"startDate" => {
                                    entry.start_date = attr.unescape_value()?.into_owned();
                                    entry.time = format_time(&entry.start_date)?;
                                }
                                b"endDate" => entry.end_date = attr.unescape_value()?.into_owned(),
                                b"value" => entry.value = attr.unescape_value()?.into_owned(),
                                b"unit" => entry.unit = attr.unescape_value()?.into_owned(),
                                _ => {}
                            }
                        }

                        entries.push(entry);
                    }
                }

                if count % 50000 == 0 {
                    let progress = reader.buffer_position() as f64 / file_size as f64 * 100.0;
                    println!("Processed {} records. Progress: {:.2}%", count, progress);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    println!("XML processing complete. Writing JSON...");

    let json = json!({ "blood_glucose_entries": entries });
    let mut output = File::create("./output.json")?;
    serde_json::to_writer_pretty(&mut output, &json)?;

    let duration = start_time.elapsed();
    println!(
        "Conversion complete in {:.2} seconds.",
        duration.as_secs_f64()
    );
    println!("Total Records processed: {}", count);
    println!("Total Blood Glucose entries: {}", blood_glucose_count);
    println!("Conversion complete. JSON file saved as output.json");
    Ok(())
}

fn format_time(date_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let date = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")?;
    let ny_time = date.with_timezone(&New_York);
    Ok(ny_time.format("%I:%M%p").to_string().to_lowercase() + " est")
}
