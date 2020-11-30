use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File :: open("file.txt");

    let mut f = match f {
        Ok(file) => {
            println!("{}", "File is exist");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the fie: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => println!("{}", s),
        Err(e) => panic!("{}", e),
    }

    println!("{}", s);
}