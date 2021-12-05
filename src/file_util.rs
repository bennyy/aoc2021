use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn file_to_string(file_name: String) -> String {
    let path = Path::new(&file_name);
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.to_string()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_data_string = String::new();
    match file.read_to_string(&mut file_data_string) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.to_string()),
        Ok(_) => {}
    }
    file_data_string
}

pub fn convert_string_of_ints_to_list(input_str: String) -> Vec<i32> {
    // Convert list of strings into a u32 vector.
    input_str
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
