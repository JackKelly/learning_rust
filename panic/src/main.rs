use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io;

const FILENAME: &str = "hello.txt";


fn main() {
    let greeting_file_result = File::open(FILENAME);

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(FILENAME) {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating {FILENAME}: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening {FILENAME}: {:?}", other_error);
    //         },
    //     },
    // };

    let greeting_file = greeting_file_result.unwrap_or_else(create_file);
}

fn create_file(error: io::Error) -> File {
    if error.kind() == ErrorKind::NotFound {
        File::create(FILENAME).unwrap_or_else(|error| {
            panic!("Problem creating {FILENAME}: {:?}", error);
        })
    } else {
        panic!("Problem opening the {FILENAME}: {:?}", error);
    }
}