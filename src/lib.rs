pub mod opts;
mod types;
pub use types::*;

pub struct Config {
    pub server: String,
    pub list_id: String,
}

impl Config {
    pub fn path(&self) -> String {
        format!("{}/api/{}", self.server, self.list_id)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: "https://list.tilman.ninja".to_owned(),
            list_id: "Demo".to_owned(),
        }
    }
}
