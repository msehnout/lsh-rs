use std::io;
use std::process::Command;

fn lsh_loop() {
    println!("Running command loop");

    loop {
        println!("lsh at uname $ ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // collect into vector
        let mut tokens : Vec<&str> = input.split_whitespace().collect();

        // iter over vector
        // for i in iter {
        //     if i == "exit" {
        //         return;
        //     }
        //     println!("{}", i);
        // }
        // run the program
        // use std::process::Command;

        // lsh_execute
        
        if tokens.len() > 0 {
            if tokens[0] == "exit" {
                return;
            } else {
                // let output = Command::new(tokens[0]).output().unwrap();
                // print!("{}", String::from_utf8_lossy(&output.stdout));
                match Command::new(tokens[0]).output() {
                    Ok(output) => print!("{}", String::from_utf8_lossy(&output.stdout)),
                    Err(_) => println!("Error occured!"),
                }
            }

        } else { 
            return;
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
