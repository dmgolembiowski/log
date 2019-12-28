use rusqlite::{Connection, Error as RusqliteError, params};

#[derive(Clone, Debug)]
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
            VALUES (?1, ?2)", self.table_name); 
        match sqldb.execute(&*exec_string, params![self.line, self.line_number],) {
            Ok(_inserted) => {()},
            Err(_e) => {
                println!("The sqlite database file could not have its records added. Closing.");
                panic!("Error at line 33: src/record/abstraction.rs");
            }
        }
    }
    pub fn update_sqlite(self, sqldb: &Connection) {
        /* UPDATE table
         * SET column_1 = new_value_1,
         *     column_2 = new_value_2
         * WHERE 
         *     search_condition;
         */
        let update_stmt = format!(
            "UPDATE {}
            SET line = ?1
            WHERE line_number = ?2",
            self.table_name);
        match sqldb.execute(&*update_stmt, params![self.line, self.line_number],) {
            Ok(_updated) => {()},
            Err(_e)      => {
                println!("The sqlite database file could not have its records updated.");
                panic!("Error at line 40 of src/record/abstractions.rs");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LineRows {
    pub rows : Vec<SQLRecord>
}
impl LineRows {
    pub fn new(line_rows: Vec<SQLRecord>) -> LineRows {
        LineRows {rows: line_rows}
    }
}


