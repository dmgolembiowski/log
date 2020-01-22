#![allow(dead_code)]
//use log::{argparse, conf, sqlite, ledger};
use log::argparse;
use log::ledger;
use log::conf::{ConfFile, ledger_conffile_abspath};

pub fn generate_temp_file(ledger_name: String) -> String {
    ledger_name
}
fn main() {
    /*
        First thing that has to occur is 
        a function must match one of four patterns 
        presented by the system arguments:
        open_ledger(String::from(""));
    
    */

    let sys_argv = argparse::get_sysargs();
    println!("You selected: {:?}", sys_argv);

    // Check ledgerConf.json file if `USE_DEDICATED_SERVER` option is set to `true`
    let configuration_file_path: String = ledger_conffile_abspath();
    let json_config = ConfFile::load(&*configuration_file_path)
        .expect("The config file could not be read. Exiting ...");

    /*
     Must ask the configuration file whether
     the user prefers to use a database server instead
     of a sqlite database file.

     We'll create a mapper for a decisive match statement.
    */
    let server_option = String::from("USE_DEDICATED_SERVER");
    let using_server = &json_config
        .get(&*server_option)
        .unwrap()
        .as_bool();
    
    /* Dispatch most common case - sqlite */
    match using_server {
        Some(false) => {
            let sys_argv: Vec<Option<String>> = argparse::get_sysargs();
            println!("{:?}", sys_argv);    
        }

    //ledger::open_ledger_from_sqlite(&String::from("LEDGER"));
}
}
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_(){}
}
