use std::path;

// TODO 1 - fix this function
pub fn divide(a: i32, b: i32) -> Option<isize> {
    if b == 0 {
        None
    }
    else {
        Some((a/b) as isize)
    }
}

// TODO 3 - fix this function
pub fn divide_error(a: i32, b: i32) -> Result<isize, String> {
    if b == 0 {
        Err(String::from("Division by 0..."))
    }
    else {
        Ok((a/b) as isize)
    }
}

// TODO 5 - list files in the diretcory
//  - if the directory does not exist, return None
fn list_dir(path: &str) -> Option<Vec<String>> {
    let path = path::Path::new(path);
    if path.is_dir() {
        Some(path.read_dir().unwrap().map(|dir| String::from(dir.unwrap().path().to_str().unwrap())).collect())
    }
    else {
        None
    }
}

pub fn run() {
    // TODO 2 - make the print work, use match and/or if let
    match divide(5, 2) {
        Some(x) => println!("Division: {}", x),
        None => println!("Division by 0..."),
    }

    // TODO 4 - make the print work, use match and/or if let
    match divide_error(5, 0) {
        Ok(x) => println!("Division: {}", x),
        Err(s) => println!("Error: {}", s),
    }


    // TODO 6 - use the list_directory function to print the current directory
    println!("{:?}", list_dir(".").unwrap());
}
