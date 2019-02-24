use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct WordFactor {
    word : String,
    rankpoint : usize,
    count : usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    match args.len() {
        4 => {
            let loadfile = &args[1];
            let savefile = &args[2];
            let keyword = &args[3];

            let con  = loadfilef(loadfile.to_string());
            let words  = loadfilef(keyword.to_string());
            
            let re = count_word(&con, &words);
            //println!("{:#?}",r);
            savefilef(savefile.to_string(), &re);
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

fn count_word (content :&String, words : &String) -> std::vec::Vec<WordFactor> {
    let spword: Vec<&str> = words.split("\r\n").collect();
    //println!("{:#?}",spword);
    let mut result: Vec<WordFactor> = Vec::with_capacity(spword.len() as usize);

    for (i, word) in spword.iter().filter(|w| w.len() > 1).rev().enumerate() {
        //println!("{} : {:#?}",i,word);
        result.push(WordFactor{
            word: word.to_string(), 
            rankpoint: (i+1) * 100, 
            count: content.clone().split(word).count() - 1,
        });
    }
    result
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
        Ok(_) => println!("Load Complete.")
    };
    contents
}

fn savefilef (filename:String, content:&Vec<WordFactor>) {
    let path = Path::new(&filename);
    let display = path.display();
    let strings = content.iter()
        .map(|a| format!("{}:{}", a.word, (a.rankpoint * a.count).to_string()))
        .collect::<Vec<_>>().join("\r\n");
    let mut file = match File::create(&path) {
    Err(e) => panic!("couldn't create {}:{}",
                        display,
                        e.description()),
    Ok(file) => file,
    };

    match file.write_all(strings.as_bytes()) {
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