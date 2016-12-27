use std::collections::HashMap;
use std::error::Error;
use std::io;

use Result;
use resource::{Resource, Translation};

extern crate csv;

pub fn parse_reader<'a, R: io::Read>(reader: R) -> Result<HashMap<String, Resource>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let headers = csv_reader.headers()
        .map_err(|e| format!("unable to parse headers, {}", e.description()))?;
    let records = csv_reader.records()
        .enumerate()
        .map(|(i, row)| {
            let row =
                row.map_err(|e| format!("unable to parse line {}, {}", i + 2, e.description()))?;
            // create an iterator over the headers and column values
            let key_value_iter = headers.iter().zip(row.into_iter());
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
                .map_err(|e| format!("unable to parse line {}, {}", i + 2, e))
        })
        .collect::<Result<Vec<Record>>>()?;
    let mut resources: HashMap<String, Resource> = HashMap::new();

    for record in records {
        let resource = resources.entry(record.name).or_insert(Resource {
            description: record.description,
            plural: record.quantity.is_some(),
            translations: HashMap::new(),
        });

        for (key, value) in record.translations.into_iter() {
            let translation = resource.translations.entry(key).or_insert(Translation::default());

            if let Some(quantity) = record.quantity.as_ref() {
                match quantity.to_lowercase().trim() {
                    "zero" | "0" => translation.zero = Some(value),
                    "one" | "1" => translation.one = Some(value),
                    "two" | "2" => translation.two = Some(value),
                    "few" => translation.few = Some(value),
                    "many" => translation.many = Some(value),
                    "other" | "?" => translation.other = Some(value),
                    _ => return Err(format!("invalid quantity \"{}\"", quantity)),
                }
            } else {
                translation.other = Some(value);
            }
        }
    }

    Ok(resources)
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
            name: self.name.ok_or(String::from("name is required but missing"))?,
            description: self.description,
            quantity: self.quantity,
            translations: self.translations,
        })
    }
}
