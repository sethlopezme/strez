use std::collections::HashMap;

#[derive(Debug)]
pub struct Resource {
    pub description: Option<String>,
    pub plural: bool,
    pub translations: HashMap<String, Translation>,
}

#[derive(Debug, Default)]
pub struct Translation {
    pub zero: Option<String>,
    pub one: Option<String>,
    pub two: Option<String>,
    pub few: Option<String>,
    pub many: Option<String>,
    pub other: Option<String>,
}