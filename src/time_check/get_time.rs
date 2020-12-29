//! This module includes only one function, get_time which get current local time from
//! server.


pub mod time_check{
    use std::process::{Command, Output};
    use std::io;
    use std::io::Write;

    /// The function gets the time from the environment variabels of date and time and the
    /// tzutil command which gets the timezone of the server.
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