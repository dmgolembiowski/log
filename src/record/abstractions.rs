use rusqlite::{Connection, Error as RusqliteError, params};
#[derive(Debug)]
pub struct SQLRecord {
    table_name :  String,
    line        : String,
    line_number : u32,
}

impl SQLRecord {
    pub fn new(
           table_name: String,
           line:       String,
           line_number:u32,
           ) -> SQLRecord {
        SQLRecord { 
            table_name: table_name, 
            line: line, 
            line_number: line_number 
        }
    }
    pub fn insert_sqlite(self, sqldb: &Connection) {
        let exec_string = format!(
            "INSERT INTO {} (line, line_number)
            VALUES({}, {});", 
            self.table_name,
            self.line,
            self.line_number);
        match sqldb.execute(&*exec_string, params![],) {
            Ok(_inserted) => {
                println!("The database was successfully populated.");
            },
            Err(_e) => {
                println!("The sqlite database file could not have its records added. Closing.");
                panic!("Error at line 33: src/record/abstraction.rs");
            }
        }
    }
}


