#[derive(Debug)]
pub struct SQLRecord {
    table_name :  String,
    line        : String,
    line_number : u32
}

impl SQLRecord {
    pub fn new(table_name: String,
           line:       String,
           line_number:u32,
           ) -> SQLRecord {
        SQLRecord { 
            table_name: table_name, 
            line: line, 
            line_number: line_number 
        }
    }
}


