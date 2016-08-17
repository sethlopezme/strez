pub enum StringResource {
    Singular {
        name: &str,
        description: &str,
        value: &str,
    },
    Plural {
        name: &str,
        description: &str,
        zero: Option<String>,
        one: Option<String>,
        two: Option<String>,
        few: Option<String>,
        many: Option<String>,
        other: Option<String>,
    }
}