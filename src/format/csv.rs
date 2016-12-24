use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::path::Path;

use Result;

extern crate csv;

pub fn parse_reader<'a, R: io::Read>(reader: R) -> Result<()> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let headers = csv_reader.headers()
                            .map_err(|e| format!("unable to parse headers, {}", e.description()))?;
    let records = csv_reader.records()
                          .enumerate()
                          .map(|(i, line)| {
                              let line = line.map_err(|e| format!("unable to parse record on line {}, {}", i + 2, e.description()))?;
                              let key_value_iter = headers.iter().zip(line.iter());
                              let mut record_builder = RecordBuilder::new();

                              for (key, value) in key_value_iter {
                                  let _ = match key.to_lowercase().trim() {
                                      "name" => record_builder.name(value.as_str()),
                                      "description" => record_builder.description(value.as_str()),
                                      "quantity" => record_builder.quantity(value.as_str()),
                                      _ => record_builder.translation(key.as_str(), value.as_str()),
                                  };
                              }

                              record_builder.build()
                          })
                          .collect::<Result<Vec<Record>>>()?;

    Err("parse_file for csv is not complete".to_string())
}

#[derive(Debug)]
struct Record<'a> {
    name: &'a str,
    description: Option<&'a str>,
    quantity: Option<&'a str>,
    translations: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
struct RecordBuilder<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    quantity: Option<&'a str>,
    translations: HashMap<&'a str, &'a str>,
}

impl<'a> RecordBuilder<'a> {
    fn new() -> RecordBuilder<'a> {
        RecordBuilder {
            name: None,
            description: None,
            quantity: None,
            translations: HashMap::new(),
        }
    }

    fn name(&mut self, name: &'a str) -> &mut Self {
        if !name.trim().is_empty() {
            self.name = Some(name);
        }

        self
    }

    fn description(&mut self, description: &'a str) -> &mut Self {
        if !description.trim().is_empty() {
            self.description = Some(description);
        }

        self
    }

    fn quantity(&mut self, quantity: &'a str) -> &mut Self {
        if !quantity.trim().is_empty() {
            self.quantity = Some(quantity);
        }

        self
    }

    fn translation(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.translations.insert(key, value);
        self
    }

    fn build(self) -> Result<Record<'a>> {
        Ok(Record {
            name: self.name.ok_or(String::from("missing name"))?,
            description: self.description,
            quantity: self.quantity,
            translations: self.translations,
        })
    }
}