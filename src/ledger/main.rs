use crate::sqlite;
use crate::mysql;
use crate::conf::{ConfFile, ledger_conffile_abspath};
use crate::record::SQLRecord;
use crate::vim;
pub fn open_ledger_from_sqlite(ledger_name: &String) {

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
       Some(true) => mysql::_connect_external_db(&ledger_name.to_string()),
        _         => {
                        // Next, we extract the path to the sqlite database

                        let sqlite_path = String::from(
                            *&json_config
                            .get(&*String::from("SQLITE_DATABASE_PATH"))
                            .unwrap()
                            .as_str()
                            .unwrap()
                        );
                        println!("sqlite_path = {:?}", &sqlite_path); // And connect to the local database file

                        let sqldb: rusqlite::Connection = sqlite::open_sqlite_connection(&sqlite_path);
                        
                        // Getting in some practice with booleans
                        if !sqlite::check_table_name(&sqldb, &ledger_name) {
                            // table does not exist yet
                            sqlite::build_first_sqlite_table(&sqldb);
                        }
                        // Next pull down the database and make a file from it
                        vim::generate_temp_file_from_sqlite(&sqldb, &ledger_name);
        }
    };
}

pub fn save_ledgers_to_db() {}

pub fn translate_sql_resp_to_file() {}


pub fn check_before_db_creation(_confpath: &String) {
    ()    
}

