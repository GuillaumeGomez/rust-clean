//   Copyright 2016 Gomez Guillaume
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::fs;
use std::path::Path;
use std::str::FromStr;

struct CleanOptions {
    recursive: bool,
    verbose: bool,
    confirmation: bool,
    level: u32
}

fn ask_confirmation(file: &Path) -> bool {
    if let Some(filename) = file.to_str() {
        loop {
            print!("clean: remove \x1b[37;1m'{}'\x1b[0m (y/n) ? ", filename);
            let mut s = String::new();

            match std::io::stdin().read_line(&mut s) {
                Ok(_) => {
                    let tmp_s = s.replace("\r\n", "").replace("\n", "");

                    if tmp_s == "y" || tmp_s == "yes" {
                        return true;
                    } else if tmp_s == "n" || tmp_s == "no" {
                        return false;
                    }
                }
                Err(_) => {}
            }
        }
    } else {
        println!("Unknown error on '{:?}'...", file);
        false
    }
}

fn start_clean(options: &CleanOptions, entry: &Path, level: u32) {
    let m = match ::std::fs::metadata(&entry) {
        Ok(e) => e,
        Err(e) => {
            println!("An error occured on '{:?}': {}", entry, e);
            return
        }
    };
    let entry_name = match entry.to_str() {
        Some(n) => n,
        None => {
            println!("Invalid entry '{:?}'", entry);
            return
        }
    };

    if m.is_file() || m.is_dir() {
        if m.is_dir() {
            if (options.recursive || entry_name == ".") &&
                (options.level == 0 || level <= options.level) {
                match fs::read_dir(entry) {
                    Ok(res) => {
                        if options.verbose {
                            println!("\x1b[36;1m-> Entering {}\x1b[0m", entry_name);
                        }
                        for tmp in res {
                            match tmp {
                                Ok(current) => {
                                    start_clean(options, &current.path(), level + 1);
                                },
                                Err(e) => println!("Error: {:?}", e)
                            };
                        }
                        if options.verbose {
                            println!("\x1b[34;1m<- Leaving {}\x1b[0m", entry_name);
                        }
                    }
                    Err(e) => {
                        println!("\x1b[31;1mProblem with directory '{}': {}\x1b[0m", entry_name, e);
                    }
                }
            }
        } else {
            match entry.file_name() {
                Some(s) => {
                    match s.to_str() {
                        Some(ss) => if ss.ends_with("~") {
                            if !options.confirmation || ask_confirmation(&Path::new(s)) {
                                match fs::remove_file(entry) {
                                    Ok(_) => {
                                        if options.verbose {
                                            println!("\x1b[32;1m{} deleted\x1b[0m", entry_name);
                                        }
                                    }
                                    Err(e) => {
                                        println!("\x1b[31;1mProblem with this file: {} -> {}\x1b[0m", entry_name, e);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
                None => {
                    println!("\x1b[31;1mProblem with this file: {}\x1b[0m", entry_name);
                }
            }
        }
    } else {
        println!("\x1b[31;1mProblem with this entry: {}\x1b[0m", entry_name);
    }
}

fn print_help() {
    println!("./clean [options] [files | dirs]");
    println!("    -r          : recursive mode");
    println!("    -v          : verbose mode");
    println!("    -i          : prompt before every removal");
    println!("    -l=[number] : Add a level for recursive mode");
    println!("--help : print this help");
}

fn main() {
    let mut args = Vec::new();

    for argument in std::env::args() {
        args.push(argument);
    }
    let mut options = CleanOptions{recursive: false, verbose: false, confirmation: false, level: 0};
    let mut files = Vec::new();

    args.remove(0);
    for tmp in args.iter() {
        if tmp.clone().into_bytes()[0] == '-' as u8 {
            let mut tmp_arg = tmp.to_owned();

            tmp_arg.remove(0);
            if tmp_arg.len() > 0 {
                for character in tmp_arg.into_bytes().iter() {
                    match *character as char {
                        '-' => {
                            if &*tmp == "--help" {
                                print_help();
                                return;
                            }
                        }
                        'r' => {
                            options.recursive = true;
                        }
                        'v' => {
                            options.verbose = true;
                        }
                        'i' => {
                            options.confirmation = true;
                        }
                        'l' => {
                            if tmp.len() < 4 || &tmp[0..3] != "-l=" {
                                println!("The \"-l\" option has to be used like this:");
                                println!("clean -r -l=2");
                                return;
                            }
                            options.level = match u32::from_str(&tmp[3..]) {
                                Ok(u) => u,
                                Err(_) => {
                                    println!("Please enter a valid number!");
                                    return;
                                }
                            };
                            println!("Level is set to {}", options.level);
                            break;
                        }
                        _ => {
                            println!("Unknown option: '{}', to have the options list, please launch with '-h' option", *character as char);
                            return;
                        }
                    }
                }
            }/* else {
                files.push(Path::new(tmp));
            }*/
        } else {
            files.push(Path::new(tmp));
        }
    }
    if files.len() == 0 {
        files.push(Path::new("."));
    }
    if options.verbose {
        println!("\x1b[33;1m=== VERBOSE MODE ===\x1b[0m");
    }
    for tmp in files.iter() {
        start_clean(&options, tmp, 0);
    }
    if options.verbose {
        println!("\x1b[33;1mEnd of execution\x1b[0m");
    }
}