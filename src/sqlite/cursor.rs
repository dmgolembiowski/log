use rusqlite::{Connection, Error as RusqliteError, params};

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

pub fn validate_connection(result_sqldb: Result<Connection, RusqliteError>) -> Connection {
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
// pub fn close_sqlite_connection(sqldb: Connection) -> Result<(), (Connection, rusqlite::Error)> 
pub fn close_sqlite_connection(sqldb: Connection) -> Result<(), (Connection, RusqliteError)> 
{
    sqldb.close() 
}


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


use crate::record::SQLRecord;
pub fn build_first_sqlite_table(sqldb: &Connection) { 
    
    // Make the table
    
    let mut line_no    = 1;
    create_table(sqldb, String::from("LEDGER"));
    let mut rows: Vec<SQLRecord> = vec!();
    
    // Open the template file and iterate over the lines to make two SQLRecords
    
    if let Ok(lines) = read_lines("templates/ledger.log") {
        for line in lines {
            if let Ok(row) = line {
                println!("{}", &row);
                rows.push(SQLRecord::new(String::from("LEDGER"), row, line_no));
                line_no += 1;
            }
        }
    }

    // Now must insert each of the SQLRecords into the SQLite database:

}
