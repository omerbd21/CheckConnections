pub mod time_check{
    use std::process::{Command, Output};
    use std::io;
    use std::io::Write;

    pub fn get_time() -> String{
        let mut time = String::from("");
            let output =  Command::new("cmd")
                .args(&["/C", "echo %date% %time% & tzutil /g"])
                .output()
                .expect("failed to execute process");
        time = String::from_utf8(output.stdout.to_vec()).unwrap();
        return time;
    }
}