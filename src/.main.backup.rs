#![allow(dead_code)]
use rusqlite;
use rusqlite::{params, Connection};
use rusqlite::Result as SQLiteResult;
use std::process::Command;
use std::boxed::Box;
use serde_json;
use std::error::Error;
use std::fs;
use serde_json::{Value, Map};
use std::env;
//use std::fs::File;
//use std::path::Path;

fn _connect_external_db(ledger_name: String) {
    println!("Accessing the {} log", ledger_name);
}

#[derive(Debug)]
struct VimFile {
    ledger_name   : Option<String>,
    tmp_file_name : Option<String>,
    rawlines      : Option<String>,
    lines         : Vec<Option<String>>
}

pub struct ConfFile;
impl ConfFile {
    pub fn load(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }
}

impl VimFile {
    fn new(ledger_name: Option<String>,
           tmp_file_name: Option<String>,
           rawlines: Option<String>,
           lines: Vec<Option<String>>) -> VimFile {
        VimFile {ledger_name, tmp_file_name, rawlines, lines}
    }
}

fn open_configuration_file() {}

fn get_ledger_by_name() {}

#[derive(Debug)]
struct SQLRecord {
    table_name : Option<String>,
    line        : Option<String>,
    line_number : Option<u32>
}

impl SQLRecord {
    fn new(table_name: Option<String>,
           line:       Option<String>,
           line_number:Option<u32>,
           ) -> SQLRecord {
        SQLRecord { 
            table_name: table_name, 
            line: line, 
            line_number: line_number 
        }
    }
}

fn translate_sql_resp_to_file() {}

fn generate_temp_file(ledger_name: String) -> String {
    ledger_name
}

fn save_ledgers_to_db() {}

fn create_table(sqldb: &Connection, table_name: String) {
    let exec_string = format!( 
        "CREATE TABLE {} (
                line        TEXT,
                line_number INTEGER
                )", table_name);
    match sqldb.execute(&*exec_string, params![],) {
        Ok(_created) => (),
        Err(_err)    => ()
    }
}
 

fn open_sqlite_connection(abspath: &String) -> SQLiteResult<Connection> {
    let sqldb = Connection::open(abspath)?;
    // Autocommit mode is on by default
    Ok(sqldb)
}

fn create_new_sqlite(abspath: &String) -> SQLiteResult<Connection> {
    open_sqlite_connection(abspath)
}


fn close_sqlite_connection(sqldb: Connection) -> Result<(), (Connection, rusqlite::Error)> 
{
    sqldb.close() 
}

fn ledger_conffile_abspath() -> Result<String, env::VarError>{
    env::var("LEDGER_CONF_PATH")
}

fn open_ledger(ledger_name: String) {
    // Get the configuration file path from the OS environment
    let ledger_conf_file: Result<String, env::VarError> = ledger_conffile_abspath();
    let configuration_file_path = ledger_conf_file
        .expect("`$LEDGER_CONF_PATH` variable is not available. Exiting.");
    
    // First read in the configuration file
    /*let config_filepath_obj = Path::new(&configuration_file_path);
    let config_file         = File::open(config_filepath_obj)
                              .expect("Configuration File not found");
    */

    // Need to transform `String` --> `&str`
    //let json_config: Result<Map<String, Value>, 
    //                            Box<dyn Error>> = ConfFile::load(&*configuration_file_path)
    let json_config = ConfFile::load(&*configuration_file_path)
        .expect("The contents of the file could not be processed properly. Exiting.");

    /*
     Must patiently ask the configuration file whether
     the user prefers to use a database server instead
     of a sqlite database file.
    */
    match &json_config.get(&*String::from("USE_DEDICATED_SERVER")).unwrap().as_bool() {
       Some(true)  => _connect_external_db(ledger_name), // will probably use arguments in the future
        _          => ()
    } 

    // Next, we extract the path to the sqlite database
    // let sqlitePath: String = *json_config.get("sqlite_abspath").unwrap()
    //    .get("sqlite_database_abspath").unwrap();
}
fn open_normal_vim() {
    Command::new("vim").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
}

fn open_vsplit_vim() {
    Command::new("vim").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
}

fn open_hsplit_vim() {
    Command::new("vim").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
}

fn check_before_db_creation(_confpath: &String) {
    ()    
}

fn initialize() {
    ()
}

fn main(){
    open_ledger(String::from(""));
}
