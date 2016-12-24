use std::collections::HashMap;
use std::error::Error;
use std::io;

use Result;

extern crate csv;

pub fn parse_reader<'a, R: io::Read>(reader: R) -> Result<()> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let headers = csv_reader.headers()
                            .map_err(|e| format!("unable to parse headers, {}", e.description()))?;
    let records = csv_reader.records()
                            .enumerate()
                            .map(|(i, line)| {
                                let line_number = i + 2;
                                let line = line.map_err(|e| format!("unable to parse line {}, {}", line_number, e.description()))?;
                                let key_value_iter = headers.iter().zip(line.into_iter());
                                let mut record_builder = RecordBuilder::new();

                                for (key, value) in key_value_iter {
                                    let _ = match key.to_lowercase().trim() {
                                        "name" => record_builder.name(value),
                                        "description" => record_builder.description(value),
                                        "quantity" => record_builder.quantity(value),
                                        _ => record_builder.translation(key.clone(), value),
                                    };
                                }

                                record_builder.build()
                                              .map_err(|string| format!("unable to parse line {}, {}", line_number, string))
                            })
                            .collect::<Result<Vec<Record>>>()?;

    println!("records = {:#?}", records);

    Err("parse_file for csv is not complete".to_string())
}

#[derive(Debug)]
struct Record {
    name: String,
    description: Option<String>,
    quantity: Option<String>,
    translations: HashMap<String, String>,
}

#[derive(Debug)]
struct RecordBuilder {
    name: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    translations: HashMap<String, String>,
}

impl RecordBuilder {
    fn new() -> RecordBuilder {
        RecordBuilder {
            name: None,
            description: None,
            quantity: None,
            translations: HashMap::new(),
        }
    }

    fn name(&mut self, name: String) -> &mut Self {
        if !name.trim().is_empty() {
            self.name = Some(name);
        }

        self
    }

    fn description(&mut self, description: String) -> &mut Self {
        if !description.trim().is_empty() {
            self.description = Some(description);
        }

        self
    }

    fn quantity(&mut self, quantity: String) -> &mut Self {
        if !quantity.trim().is_empty() {
            self.quantity = Some(quantity);
        }

        self
    }

    fn translation(&mut self, key: String, value: String) -> &mut Self {
        self.translations.insert(key, value);
        self
    }

    fn build(self) -> Result<Record> {
        Ok(Record {
            name: self.name.ok_or(String::from("missing name"))?,
            description: self.description,
            quantity: self.quantity,
            translations: self.translations,
        })
    }
}