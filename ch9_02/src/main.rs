use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = File::open("meh.txt");

    let fileMeh = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create("meh.txt") {
                        Ok(file) => file,
                        Err(e) => {
                            panic!("Problem creating the file: {:?}", e);
                        }
                    }
                },
                other_error =>  panic!("Problem opening the file: {:?}", other_error)
                
            }
        }
    };
}