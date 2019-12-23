use crate::argparse;
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

