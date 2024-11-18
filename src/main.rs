use std::fs::File;
use std::io::{stdin, BufReader};
use std::io::prelude::*;
use std::ops::Index;
use std::path::Path;

fn main() -> std::io::Result<()> {//the Result() thing says that main() might error (see last line)
    let mut file_name = String::new();
    let mut lines = Vec::new();

    println!("Enter the path (enter file name if file is in main directory)");
    stdin().read_line(&mut file_name).expect("Failed to read line");

    //this code auto-completed itself; I have no idea how it works
    let path = Path::new(file_name.trim_end());
    let file = File::open(path)?;//I think the '?' makes the errors come up automatically without having to handle them in match statements

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line{//need a match statement here because specific things are happening, not just assigning variables.
            Ok(line) => {lines.push(line)},
            Err(e) => { println!("Failed to read line: {}", e); }
        }
    }

    if lines.len() % 2 != 0{
        print!("{}", lines.index((lines.len()-1)/2));
    }else{
        //parsing the strings to floats
        let num1: f64 = lines.index((lines.len()-1)/2).parse().expect("Failed to parse line");
        let num2: f64 = lines.index(((lines.len()-1)/2)+1).parse().expect("Failed to parse line");

        println!("{}", (num1 + num2)/2.0);
    }

    Ok(())//main() ran successfully
}
