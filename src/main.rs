use std::env;
use std::fs;
use std::io::Write;
use chrono;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let dir_name = String::from(".");
        make_python_files(&dir_name);
        make_info_file(&dir_name);
        make_input_file(&dir_name);
    } else {
        for arg in args.iter() {
            if arg == &args[0] { continue; }
    
            let dir_name = arg;
            make_directory(&dir_name);
            make_python_files(&dir_name);
            make_info_file(&dir_name);
            make_input_file(&dir_name);
        }
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
        let name = format!("{}/{}.py", dir_name, question_name);
        let file_exists = fs::metadata(&name).is_ok();
        if file_exists == true{ continue; }

        println!("creating file: {}", &name);
        fs::File::create(&name).unwrap();
    }
}

fn make_info_file(dir_name: &String) {
    let name = format!("{}/note.txt", dir_name);

    let file_exists = fs::metadata(&name).is_ok();
    if file_exists == true { return; }

    println!("creating file: {}", &name);
    let mut file = fs::File::create(&name).unwrap();
    let now = chrono::Local::now().format("%Y/%m/%d %H:%M").to_string();
    writeln!(file, "{}", dir_name).unwrap();
    writeln!(file, "Created at {}", now).unwrap();
}

fn make_input_file(dir_name: &String) {
    let name = format!("{}/input.txt", dir_name);

    let file_exists = fs::metadata(&name).is_ok();
    if file_exists == true { return; }

    println!("creating file: {}", &name);
    fs::File::create(&name).unwrap();
}