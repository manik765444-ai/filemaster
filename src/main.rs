use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    // Handle command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a directory was provided, otherwise use the current directory
    let dir = if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    // Read the directory contents
    match fs::read_dir(dir) {
        Ok(paths) => {
            // Iterate over the directory entries
            for path in paths {
                match path {
                    Ok(entry) => {
                        // Print the file name
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                    Err(err) => {
                        // Handle error in reading a directory entry
                        eprintln!("Error: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            // Handle error in reading the directory
            eprintln!("Error: {}", err);
        }
    }
}