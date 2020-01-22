use crate::argparse;
use rusqlite::Connection;
#[derive(Debug)]
pub struct VimFile {
    ledger_name   : Option<String>,
    tmp_file_name : Option<String>,
    rawlines      : Option<String>,
    lines         : Vec<Option<String>>
}

impl VimFile {
    fn new(ledger_name: Option<String>,
           tmp_file_name: Option<String>,
           rawlines: Option<String>,
           lines: Vec<Option<String>>) -> VimFile {
        VimFile {ledger_name, tmp_file_name, rawlines, lines}
    }
}

pub fn vim_render(sys_argv: Vec<Option<String>>) -> Vec<Option<String>>{
    // Use information provided by std::env::args
    //use std::env::{args as sysargs, Args as Sysargs};
    let sys_argv = argparse::get_sysargs(); // The iterator

    /*Cases:
      1) log
      2) log algebra-notes
      3) log -o geometry sociology
      4) log -O geometry sociology
      5) log geometry sociology
      6) log crabmeat lightbulb eyegoop
    
    try:
        sas = sys_argv.next()
        assert((sas != "-o" || sys_argv != "-O"))
    except AssertionError:
        print("Not splitting windows")
        if [[ "$sas" == "-o" ]]; then
            sys_argv
    */

}

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn load_tmp_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file could be found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
use crate::record::SQLRecord;
pub fn save_tmp_file_to_sqlite(
    filename: impl AsRef<Path>, 
    ledger_name: &String,
    sqldb: &Connection) {
    
    // First gather lines from the tmp file with the given name
    let mut records: Vec<SQLRecord> = vec!();
    let mut line_no = 1;  
    let lines: Vec<String> = load_tmp_file(&filename);
     
    // Next save the lines to the sqlite instance
    for line in lines {
        if let row = line {
            records.push(SQLRecord::new(ledger_name.to_string(), row, line_no));
            line_no += 1;
        }
    }
    for sql_record in records {
        sql_record.update_sqlite(sqldb);
    }
    // Finally, remove the temporary file from the tmp/ folder
    match std::fs::remove_file(filename) {
        Ok(_worked) => {()},
        Err(_e) => {
            println!("Failed to remove the file! Run `rm tmp/LEDGER.log`");
        }
    }
}


use crate::sqlite;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn tmp_file_header(ledger_name: &String) -> () {
    let mut title_length: u32 = 0;
    for _character in ledger_name.chars() {
        title_length += 1;
    }
    println!("The value of title_length = {:?}", title_length);
}

pub fn generate_temp_file_from_sqlite(
    sqldb: &Connection,
    ledger_name: &String,
    local_tmp_folder: &String) -> String {

    // Step 1: Query the database for `table_name` := ledger_name 
    let ledger_lines: Vec<(String, i32)> = sqlite::open_sqlite_table(sqldb, ledger_name);
    
    // Step 2: Open a temp file and write each row to its corresponding line number in the file
    let filename = String::from(format!("{}{}.log", local_tmp_folder, ledger_name));
    println!("File is {:?}", &filename);
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&filename)
        .unwrap();

    // Next iterate over each of the vector slices and append them to the file
    for line in ledger_lines {
        let (content, _line_number) = line;
        if let Err(e) = writeln!(log_file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    // Step 3: Return an absolute path to the file (Profit)
    filename 
}
