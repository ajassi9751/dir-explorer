use std::fs;

fn main() {
    // I should check if the arg is a directory or a file but its fine
    let args: Vec<String> = std::env::args().collect();
    // Maybe I should include multiple arguments but maybe not
    if 1 < args.len() {
        if args[1].contains("/") {
            println!("{}", &args[1].as_str());
        } else {
            println!("{}/", &args[1].as_str());
        }
        print_dir(&args[1].as_str(), 0);
    } else {
        println!("./");
        print_dir(".", 0);
    }
}

fn print_dir(path: &str, generation: i32) {
    let ending_part: String = "└──".to_string();
    let mut tabs: String = "".to_string();
    for _i in 0..generation {
        tabs += "|  ";
    }
    match fs::read_dir(path) {
        Ok(entries) => {
            for ientry in entries {
                let entry;
                match ientry {
                    Ok(v) => entry = v,
                    Err(e) => panic!("Illegal file/directory access error: {}", e),
                }
                if entry.file_type().unwrap().is_dir() {
                    println!(
                        "{} {}/",
                        tabs.clone() + &ending_part,
                        entry.file_name().to_string_lossy()
                    ); // I should instead use a buffered technique but this works for now
                    print_dir(entry.path().display().to_string().as_str(), generation + 1);
                } else {
                    println!(
                        "{} {}",
                        tabs.clone() + &ending_part,
                        entry.file_name().to_string_lossy()
                    );
                }
            }
        }
        Err(e) => panic!("Illegal file/directory access error: {}", e),
    }
}
