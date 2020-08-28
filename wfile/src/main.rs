use std::fs::File;
use rand::Rng;
use std::io::prelude::*;
use std::path::Path;


/* I demand a list of random numbers! */
fn create_number_list() -> String {
    
    let mut builda_string: String = "".to_string();
    let mut randy = rand::thread_rng();
    let mut x = rand::random::<u64>();

    for _i in 0..9 {
        x = randy.gen();
        builda_string.push_str(&x.to_string());
        builda_string.push_str(&",");
    }
    builda_string.push_str(&x.to_string());
   
    return builda_string;
}


fn write_stuff(hotstring: String) -> (){

    let path = Path::new("hotfile.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write to `file`, returns `io::Result<()>`
    match file.write_all(hotstring.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


fn main() -> std::io::Result<()> {

    let int_vec = create_number_list();

    write_stuff(int_vec);
    
    Ok(())
}

