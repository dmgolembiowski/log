#![allow(dead_code)]
//use log::argparse;
use rusqlite;
use rusqlite::{params, Connection};
//use rusqlite::Result as SQLiteResult;
//use std::process::Command;
use std::boxed::Box;
use serde_json;
use std::error::Error;
use std::fs;
use serde_json::{Value, Map};
use std::env;

fn _connect_external_db(ledger_name: String) {
    println!("Accessing the {} log", ledger_name);
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

fn open_configuration_file() {}

fn get_ledger_by_name() {}

#[derive(Debug)]
pub struct SQLRecord {
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
pub fn generate_temp_file(ledger_name: String) -> String {
    ledger_name
}

pub fn create_table(sqldb: &Connection, table_name: String) {
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

pub fn open_sqlite_connection(abspath: &String) -> Connection {
    let result_sqldb = Connection::open(abspath);
    // Autocommit mode is on by default
    let sqldb = validate_connection(result_sqldb);
    sqldb
}

pub fn validate_connection(result_sqldb: Result<Connection, rusqlite::Error>) -> Connection {
    match result_sqldb {
        Ok(conn) => conn,
        Err(_error) => {
            panic!("Oops, could not make SQLite DB connection.");
        }
    }
}

pub fn create_new_sqlite(abspath: &String) -> Connection {
    open_sqlite_connection(abspath)
}

pub fn check_table_name(sqldb: &rusqlite::Connection, tablename: &String) -> bool {
    match sqldb.execute(&*format!("SELECT 1 FROM {}", tablename), params![],) {
        Ok(_table_exists) => true, // Therefore, should not create new table
        Err(_err)         => false, //           should create new table
    }
}

pub fn close_sqlite_connection(sqldb: Connection) -> Result<(), (Connection, rusqlite::Error)> 
{
    sqldb.close() 
}

pub fn ledger_conffile_abspath() -> String{
    let conffile: Result<String, env::VarError> = env::var("LEDGER_CONF_PATH");
    conffile.unwrap()
}

pub fn open_ledger(ledger_name: &String) {

    // Get the configuration file path from the OS environment
    let configuration_file_path: String = ledger_conffile_abspath();
    println!("The conf file: {:?}", &configuration_file_path);

    // Need to transform `String` --> `&str`
    let json_config = ConfFile::load(&*configuration_file_path)
        .expect("The contents of the file could not be processed properly. Exiting.");

    /*
     Must ask the configuration file whether
     the user prefers to use a database server instead
     of a sqlite database file.
    */
    // Break this apart into a mapper that cases out which things need to happen
    let _ret = match &json_config.get(&*String::from("USE_DEDICATED_SERVER")).unwrap().as_bool() {
       Some(true) => _connect_external_db(ledger_name.to_string()),
        _         => {
                        // Next, we extract the path to the sqlite database
                        let sqlite_path = String::from(*&json_config
                            .get(&*String::from("SQLITE_DATABASE_PATH"))
                            .unwrap()
                            .as_str()
                            .unwrap()
                        );
                        println!("sqlite_path = {:?}", &sqlite_path);                    
                        // And connect to the local database file
                        let sqldb: rusqlite::Connection = open_sqlite_connection(&sqlite_path);
                        // Getting in some practice with booleans
                        if !check_table_name(&sqldb, &ledger_name) {
                            // table does not exist yet
                            println!("Creating first time SQLite default ledger");
                            /*
                               Call a function that opens the templates folder
                               and extracts the default ledger file. Then, process
                               it with functions into fields that can be easily manipulated
                               by the sqlite connection.
                            */
                            build_first_sqlite_table(&sqldb, &ledger_name);
                            println!("Setting up the initial configurations for the SQLite database...");
                        
                        }
                        // gen_vim_session();
        }
    };
}

pub fn build_first_sqlite_table(
    sqldb: &rusqlite::Connection,
    ledger_name: &String) {
    
}

pub fn get_sysargs() -> Vec<Option<String>> {
    use std::env::args as sysargs;
    let argv = sysargs().skip(1); // The iterator
    let mut sys_argv: Vec<Option<String>> = vec!();
    for some_arg in argv {
        sys_argv.push(Some(some_arg));
    }
    sys_argv
}

pub fn save_ledgers_to_db() {}

fn translate_sql_resp_to_file() {}


fn check_before_db_creation(_confpath: &String) {
    ()    
}

fn initialize() {
    ()
}

fn main(){
/*
    First thing that has to occur is 
    a function must match one of four patterns 
    presented by the system arguments:
    open_ledger(String::from(""));
    
*/
    let _sys_argv = get_sysargs();
    open_ledger(&String::from("ledgerTest"));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_(){}
}
