use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


/* I demand a list of random numbers! */
fn create_number_list(string_of_ints: String) -> () {
    
    //let s = String::from(string_of_ints);
    // "string".graphemes(true).count()
    let int_count = 0; 
    for c in string_of_ints.chars() {
        
    }


    
   
    return builda_string;
}


fn read_stuff(filename: String) -> String {

    let mut content = String::new();

    match File::open(filename) {
        // The file is open (no error).
        Ok(mut file) => {
            
            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            println!("{}", content);

            // The file is automatically closed when is goes out of scope.
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        },
    }

    return content;
    
}


fn main() -> std::io::Result<()> {

    let file_name = "./hotfile.txt";

    let str_of_ints = read_stuff(file_name);
    
    create_number_list(str_of_ints);

    Ok(())
}

