pub mod format;
pub mod language;
pub mod platform;
pub mod quantity;
pub mod resource;
pub mod settings;

pub type Result<T> = std::result::Result<T, String>;