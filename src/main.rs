use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    match args.len() {
        3 => {
            let loadfile = &args[1];
            let savefile = &args[2];

            let con  = loadfilef(loadfile.to_string());
            count_word(&con, "테스트");
            savefilef(savefile.to_string(), con);
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

fn count_word (content :&String, word : &str) {
    let leng = content.split(word).count() - 1;
    println!("Tested : {:?}",leng);
}

fn loadfilef (filename:String) -> String {
    let path = Path::new(&filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Open error {}:{}", display, e.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!("Read error {} : {}", display, e.description()),
        Ok(_) => println!("Load Complete."),
    };
    contents
}

fn savefilef (filename:String, content:String) {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::create(&path) {
    Err(e) => panic!("couldn't create {}: {}",
                        display,
                        e.description()),
    Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(e) => {
            panic!("couldn't write to {}: {}", display,
                                               e.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn help () {
    println!("please match arguments");
}

