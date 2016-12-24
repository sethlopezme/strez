use std::collections::HashMap;

pub enum Translation<'a> {
    Singular(&'a str),
    Plural {
        zero: &'a str,
        one: &'a str,
        two: &'a str,
        few: &'a str,
        many: &'a str,
        other: &'a str,
    }
}

pub struct Resource<'a> {
    name: &'a str,
    description: Option<&'a str>,
    quantity: Option<&'a str>,
}

pub struct ResourceBuilder<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    quantity: Option<&'a str>,
    translations: HashMap<&'a str, Translation<'a>>,
}

impl<'a> ResourceBuilder<'a> {
    fn new() -> ResourceBuilder<'a> {
        ResourceBuilder {
            name: None,
            description: None,
            quantity: None,
            translations: HashMap::new(),
        }
    }

    fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn description(&mut self, description: &'a str) -> &mut Self {
        self.description = Some(description);
        self
    }

    fn quantity(&mut self, quantity: &'a str) -> &mut Self {
        self.quantity = Some(quantity);
        self
    }
}