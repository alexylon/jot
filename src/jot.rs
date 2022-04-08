use std::fs::OpenOptions;
use std::io::Write;

pub fn write_data(words: &Vec<String>) {
    match home::home_dir() {
        Some(home_dir) => {
            let filename = format!("{}/jot.txt", home_dir.display());

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&filename);

            for (pos, word) in words.iter().enumerate() {
                if pos == 0 { continue; }
                match file {
                    Ok(ref mut file_ok) => {
                        match write!(file_ok, "{} ", word) {
                            Ok(_) => {}
                            Err(e) => { eprintln!("Couldn't write to file: {}", e) }
                        }
                    }
                    Err(ref e) => { eprintln!("Error: {}", e) }
                }
            }

            match file {
                Ok(ref mut file_ok) => {
                    match writeln!(file_ok, "") {
                        Ok(_) => {}
                        Err(e) => { eprintln!("Couldn't write to file: {}", e) }
                    }
                }
                Err(ref e) => { eprintln!("Error: {}", e) }
            }
        }

        None => println!("Impossible to get your home dir!"),
    }
}