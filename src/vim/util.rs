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
    */
    sys_argv
}
pub fn vim_save(temp_file_path: &String) {}

use crate::sqlite;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

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
