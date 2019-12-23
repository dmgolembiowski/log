#![allow(dead_code)]
//use log::{argparse, conf, sqlite, ledger};
use log::argparse;
use log::ledger;


#[derive(Debug)]
pub struct SQLRecord {
    table_name :  Option<String>,
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
fn main(){
/*
    First thing that has to occur is 
    a function must match one of four patterns 
    presented by the system arguments:
    open_ledger(String::from(""));
    
*/
    let _sys_argv = argparse::get_sysargs();
    ledger::open_ledger(&String::from("ledgerTest"));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_(){}
}
