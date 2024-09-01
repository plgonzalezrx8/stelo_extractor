use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::models::BloodGlucoseEntry;
use crate::utils::format_time;

pub fn parse_xml(input_path: &Path) -> Result<(Vec<BloodGlucoseEntry>, usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
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

                if let Some(ty) = e.attributes().filter_map(|a| a.ok()).find(|attr| attr.key.as_ref() == b"type") {
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
                                b"sourceName" => entry.source_name = attr.unescape_value()?.into_owned(),
                                b"creationDate" => entry.creation_date = attr.unescape_value()?.into_owned(),
                                b"startDate" => {
                                    entry.start_date = attr.unescape_value()?.into_owned();
                                    entry.time = format_time(&entry.start_date)?;
                                },
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

    Ok((entries, count, blood_glucose_count))
}

