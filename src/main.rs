use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        make_python_files(&String::from("."));
        return;
    }

    for arg in args.iter() {
        if arg == &args[0] { continue; }

        let dir_name = &arg.to_owned();
        make_directory(&dir_name);
        make_python_files(&dir_name);
    }

    println!("Done");
}

fn make_directory(path: &String) {
    if fs::metadata(&path).is_err() {
        println!("creating directory: {}", &path);
        fs::DirBuilder::new().create(&path).unwrap();
    }
}

fn make_python_files(dir_name: &String) {
    let mut question_names = ('a'..='g')
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    question_names.push(String::from("_ex"));

    for question_name in question_names.iter() {
        let name = dir_name.to_owned() + "/" + &question_name.to_string() + ".py";
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