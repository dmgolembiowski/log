#![allow(dead_code)]
//use log::{argparse, conf, sqlite, ledger};
use log::argparse;
use log::ledger;

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
    ledger::open_ledger_from_sqlite(&String::from("LEDGER"));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_(){}
}
