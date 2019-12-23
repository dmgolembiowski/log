use std::process::Command;
pub fn open_normal_vim(temporary_filename: &String) {
    Command::new("vim").arg(temporary_filename)
        .status()
        .expect("Something went wrong");
}

pub fn open_vsplit_vim(_temporary_filenames: Vec<String>) {
   Command::new("vim").arg("-O").arg("test")
       .status()
       .expect("Something went wrong");
}

pub fn open_hsplit_vim() {
    Command::new("vim").arg("-o").arg("/home/david/logTest.txt")
        .status()
        .expect("Something went wrong");
}

