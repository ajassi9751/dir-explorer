use ncurses::*; // Bad but fine`
use std::fs;
use std::process::exit;
mod tree;
use tree::Node;

// Could convert the program to only show the current directory like ls does and just be able to
// jump through, probalby have one default behavior and one option, also make -a and option
// I first need to organize the project to collect the directories into a data structure (a tree) instead of printing
// Then use ncurses to render the TUI and implement the jump functionality (use std::env::set_current_dir for this)
fn main() {
    // Test
    // initscr();
    // if !has_colors() {
    //     eprintln!("Terminal doesn't have color support");
    //     exit(1);
    // }
    // start_color();
    // init_pair(1, COLOR_RED, COLOR_BLUE); // This highlights the text but doesn't change the color :(
    // attron(COLOR_PAIR(1));
    // attron(A_BOLD);
    // let _ = addstr("Hi there!");
    // attroff(COLOR_PAIR(1));
    // attroff(A_BOLD);
    // getch();
    // endwin();
    // let mut _test: Tree<&str> = Tree::new("Hi");
    // _test.add_node("Hi!");
    // _test.get_node_mut(0).unwrap().add_node("Lol");
    // println!("{}", _test.get_node(0).unwrap().get_node(0).unwrap().get_val());
    let args: Vec<String> = std::env::args().collect();
    // Maybe I should include multiple arguments but maybe not
    if 1 < args.len() {
        match fs::exists(&args[1]) {
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
        let test = std::path::Path::new(&args[1]);
        if !test.is_dir() {
            eprintln!("Path entered is not a directory");
            exit(1);
        }
        if args[1].contains("/") {
            println!("\x1b[36m{}\x1b[0m", args[1]);
        } else {
            println!("\x1b[36m{}/\x1b[0m", args[1]);
        }
        print_dir(args[1].as_str(), 0);
    } else {
        println!("\x1b[36m./\x1b[0m");
        print_dir(".", 0);
    }
    let mut tree: Node<String> = Node::new(String::from("."));
    collect_dir_node(".", &mut tree);
    print_nodes(&tree, 0);
}

// Need to make another function that takes a &mut Tree but maybe tree inst even a needed data type
// fn print_dir(path: &str, generation: u32, node: &mut Node) {
fn print_dir(path: &str, generation: u32) {
    // Use this │ (Not the same as |)
    // let ending_part: String = "└──".to_string();
    let ending_part: String = "└─".to_string();
    let mut tabs: String = "".to_string();
    for _i in 0..generation {
        // tabs += "|  ";
        tabs += "   ";
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
                // Most digusting name I have seen
                let mut _isit_dir: bool = false;
                match entry.file_type() {
                    Ok(v) => _isit_dir = v.is_dir(),
                    Err(e) => {
                        eprintln!(
                            "Error determining if it is a file or directory error: {}",
                            e
                        );
                        exit(1)
                    }
                }
                if _isit_dir {
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

// I know String is bad
fn collect_dir_node (path: &str, node: &mut Node<String>) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for ientry in entries {
                let entry;
                match ientry {
                    Ok(v) => {
                        entry = v; 
                        // Most digusting name I have seen
                        let mut _isit_dir: bool = false;
                        match entry.file_type() {
                            Ok(v) => _isit_dir = v.is_dir(),
                            Err(e) => {
                                eprintln!(
                                    "Error determining if it is a file or directory error: {}",
                                    e
                                );
                                cleanup(1)
                            }
                        }
                        if _isit_dir {
                            node.add_node(entry.file_name().to_string_lossy().into_owned());
                            match node.get_node_mut(node.get_children().len()-1) {
                                Some(v) => collect_dir_node(entry.path().display().to_string().as_str(), v),
                                None => {
                                    eprintln!("Error traversing tree");
                                    cleanup(1)
                                }
                            } 
                        } else {
                            node.add_node(entry.file_name().to_string_lossy().into_owned());
                        } 
                    },
                    Err(e) => {
                        eprintln!("Illegal file/directory access error: {}", e);
                        cleanup(1);
                    },
                }
            }
        },
        Err(e) => {
            eprintln!("Illegal file/directory access error: {}", e);
            exit(1);
        }
    }
}

fn print_nodes (node: &Node<String>, generation: u32) { 
    if generation == 0 {
        println!("\x1b[36m{}/\x1b[0m", node.get_val());
        if !node.get_children().is_empty() {
            for n in node.get_children() {
                print_nodes(n, 1);
            }
        }
    }
    else {
        let gener = generation - 1;
        let ending_part: String = "└─".to_string();
        let mut tabs: String = "".to_string();
        for _i in 0..gener {
            // tabs += "|  ";
            tabs += "   ";
        }
        if node.get_children().is_empty() {
            println!("{} {}", tabs.clone() + &ending_part, node.get_val());
        }
        else {
            println!("{} \x1b[36m{}/\x1b[0m", tabs.clone() + &ending_part, node.get_val());
            // println!("{}", node.get_children().len());
            for n in node.get_children() { 
                print_nodes(n, gener + 2);
            }
        }
    }
}

// Make this a macro?
fn cleanup (error: i32) {
    endwin();
    exit(error);
}