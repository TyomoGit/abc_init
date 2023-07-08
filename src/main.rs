use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        make_python_file(&String::from("./"));
        process::exit(0);
    }

    for arg in args.iter() {
        if arg == &args[0] {
            continue;
        }

        let file_name = arg.to_owned() + "/";
        if fs::metadata(&file_name).is_err() {
            println!("creating directory: {}", &file_name);
            fs::DirBuilder::new().create(&file_name).unwrap();
        }

        make_python_file(&file_name);
    }

    println!("Done");
}

fn make_python_file(path: &String) {
    for alphabet in 'a'..='g' {
        let name = path.to_owned() + &alphabet.to_string() + ".py";
        let file_exists = match fs::metadata(&name) {
            Ok(_) => true,
            Err(_) => false,
        };

        if file_exists == false {
            println!("creating file: {}", &name);
            fs::File::create(&name).unwrap();
        }
    }
}