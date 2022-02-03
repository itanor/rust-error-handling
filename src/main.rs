use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn v1() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn v2() {
    let _f = File::open("abc.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("abc.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// propagating errors
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let f = File::open("/tmp/some.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("/tmp/some.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    v1();
    v2();
    let result = read_username_from_file_v1();
    if result.is_ok() {
        println!("read from file v1: {}", result.unwrap());
    }
    let result = read_username_from_file_v2();
    if result.is_ok() {
        println!("read from file v2: {}", result.unwrap());
    }
}
