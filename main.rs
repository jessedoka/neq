use std::io::Error;
use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};

fn init_repo() -> io::Result<()> {
    // Sets up a new .neq folder in the current directory.
    // Should check if '.neq' exists if not create one. 

    
}

fn add_file(filename: str){
    // stage a file for commit 
}

fn commit(message: str) {
}

fn view_log(){
}

fn main_menu(){
}


fn main() -> io::Result<()> {
    // Method 1: Reading the whole file into a String
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File content:\n{}", contents);

    //// Method 2: Reading line by line using a buffered reader
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

