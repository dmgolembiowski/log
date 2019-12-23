use std::boxed::Box;
use std::{fs, env};
use std::error::Error;
use serde_json::{Value,Map,from_str};

pub struct ConfFile;
impl ConfFile {
    pub fn load(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }
}
pub fn ledger_conffile_abspath() -> String{
    let conffile: Result<String, env::VarError> = env::var("LEDGER_CONF_PATH");
    conffile.unwrap()
}

