use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    match args.len() {
        3 => {
            let loadfile = &args[1];
            let savefile = &args[2];

        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

fn help () {
    println!("run as:");
    println!("rhetoric filename1.txt filename2.txt keyword.txt");
    println!(":)");
}

