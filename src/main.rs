use std::io;
use std::process::Command;
use std::env;
use std::path::Path;

fn lsh_loop() {
    println!("Running command loop");

    loop {
        println!("lsh at uname $ ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // collect into vector
        let mut tokens : Vec<&str> = input.split_whitespace().collect();
        
        if tokens.len() > 0 {
            match tokens[0] {
                "exit" => return,
                "pwd"  => {
                    let p = env::current_dir().unwrap();
                    println!("The current directory is {}", p.display());
                },
                "cd"   => {
                    let dir = Path::new(tokens[1]);
                    env::set_current_dir(&dir).unwrap();
                },
                _      => {
                    match Command::new(tokens[0])
                                  .args(&tokens[1..])
                                  .output() {
                        Ok(output) => print!("{}", String::from_utf8_lossy(&output.stdout)),
                        Err(_) => println!("Error occured!"),
                    }
                }
            }
        }

    }
}

fn main() {
    println!("Welcome in lsh!");

    // Possibly load config files
    //
    // Run command loop
    lsh_loop();
    // Shutdown
    // Rust does not have any return value
}
