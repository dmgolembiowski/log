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
//pub fn validate_connection(result_sqldb: Result<Connection, rusqlite::Error>) -> Connection {
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

pub fn build_first_sqlite_table(
    sqldb: &rusqlite::Connection,
    ledger_name: &String) {  
}
