use std::env::args as sysargs;
pub fn get_sysargs() -> Vec<Option<String>> {
    let argv = sysargs().skip(1);
    let mut sys_argv: Vec<Option<String>> = vec!();
    for some_arg in argv {
        sys_argv.push(Some(some_arg));
    }
    sys_argv
}
