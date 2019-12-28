use std::process::Command;

pub fn open_normal_vim(temporary_filename: &String) {
    let mut cmd = Command::new("vim");
    if let Ok(mut child) = cmd.arg(&temporary_filename).spawn() {
        child.wait().expect("Command was not running");
        println!("Saving...");
        // Now we have to open a file reader and save the file's lines to the database
    } else {
        println!("Vim failed to start");
    }
    /*
        .status()
        .expect("Something went wrong");
    cmd
    */
}

pub fn open_vsplit_vim(temporary_filenames: Vec<String>) -> std::process::ExitStatus {
   let cmd = Command::new("vim").arg("-O").arg("test")
       .status()
       .expect("Something went wrong");
    cmd
}

pub fn open_hsplit_vim() -> std::process::ExitStatus {
    let cmd = Command::new("vim").arg("-o").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
    cmd
}

