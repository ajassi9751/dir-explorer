use std::fs;
use std::process::exit;

fn main() {
    // I should check if the arg is a directory or a file but its fine
    let args: Vec<String> = std::env::args().collect();
    // Maybe I should include multiple arguments but maybe not
    if 1 < args.len() {
        match fs::exists(&args[1].as_str()) {
            Ok(v) => {
                if !v {
                    eprintln!("Directory doesn't exist");
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("Directory doesn't exist error: {}", e);
                exit(1);
            }
        }
        if args[1].contains("/") {
            println!("{}", &args[1].as_str());
        } else {
            println!("{}/", &args[1].as_str());
        }
        print_dir(&args[1].as_str(), 0);
    } else {
        println!("\x1b[36m./\x1b[0m");
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
                    Err(e) => {
                        eprintln!("Illegal file/directory access error: {}", e);
                        exit(1);
                    }
                }
                if entry.file_type().unwrap().is_dir() {
                    println!(
                        // "\x1b[93m{} {}/\x1b[0m",
                        "{} \x1b[36m{}/\x1b[0m",
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
        Err(e) => {
            eprintln!("Illegal file/directory access error: {}", e);
            exit(1);
        }
    }
}
