//RUST PASSWORD MANAGER

use std::io;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::io::ErrorKind;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::io::ErrorKind::InvalidData;
use std::io::ErrorKind::InvalidInput;
use std::io::ErrorKind::PermissionDenied;
use std::io::ErrorKind::AlreadyExists;
use std::io::ErrorKind::Other;
use std::io::ErrorKind::Interrupted;
use std::io::ErrorKind::TimedOut;
use std::io::ErrorKind::WouldBlock;
use std::io::ErrorKind::WriteZero;
use std::io::ErrorKind::UnexpectedEof;
use std::io::ErrorKind::ConnectionRefused;
use std::io::ErrorKind::ConnectionReset;
use std::io::ErrorKind::ConnectionAborted;
use std::io::ErrorKind::NotConnected;
use std::io::ErrorKind::AddrInUse;
use std::io::ErrorKind::AddrNotAvailable;
use std::io::ErrorKind::BrokenPipe;



fn main() {
    println!("Welcome to the Rusty Password Manager!");
    println!("Please enter your master password: ");
    let mut master_password = String::new();
    io::stdin().read_line(&mut master_password).expect("Failed to read line");
    let master_password = master_password.trim();
    let mut file_path = String::from("passwords/");
    file_path.push_str(master_password);
    file_path.push_str(".txt");
    let path = Path::new(&file_path);
    if path.exists() {
        println!("Welcome back!");
        println!("What would you like to do?");
        println!("1. View passwords");
        println!("2. Add a password");
        println!("3. Delete a password");
        println!("4. Change master password");
        println!("5. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => view_passwords(&file_path),
            2 => add_password(&file_path),
            3 => delete_password(&file_path),
            4 => change_master_password(&file_path),
            5 => println!("Goodbye!"),
            _ => println!("Invalid choice!"),
        }
    } else {
        println!("No password file found. Would you like to create one?");
        println!("1. Yes");
        println!("2. No");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => create_password_file(&file_path),
            2 => println!("Goodbye!"),
            _ => println!("Invalid choice!"),
        }
    }
}

fn view_passwords(file_path: &String) {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn add_password(file_path: &String) {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Failed to read file");
    println!("Please enter the name of the website or app: ");
    let mut website = String::new();
    io::stdin().read_line(&mut website).expect("Failed to read line");
    let website = website.trim();
    println!("Please enter the username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();
    println!("Please enter the password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim();
    let new_contents = format!("{}\nWebsite: {}\nUsername: {}\nPassword: {}", contents, website, username, password);
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(new_contents.as_bytes()).expect("Failed to write to file");
}

fn delete_password(file_path: &String) {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Failed to read file");
    println!("Please enter the name of the website or app you would like to delete: ");
    let mut website = String::new();
    io::stdin().read_line(&mut website).expect("Failed to read line");
    let website = website.trim();
    let mut new_contents = String::new();
    let mut found = false;
    for line in contents.lines() {
        if line == website {
            found = true;
        } else if found {
            found = false;
        } else {
            new_contents.push_str(line);
            new_contents.push_str("\n");
        }
    }
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(new_contents.as_bytes()).expect("Failed to write to file");
}

fn change_master_password(file_path: &String) {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Failed to read file");
    println!("Please enter your new master password: ");
    let mut new_master_password = String::new();
    io::stdin().read_line(&mut new_master_password).expect("Failed to read line");
    let new_master_password = new_master_password.trim();
    let new_file_path = format!("passwords/{}.txt", new_master_password);
    let new_path = Path::new(&new_file_path);
    if new_path.exists() {
        println!("A password file already exists with that name!");
    } else {
        let mut file = File::create(&new_file_path).expect("Failed to create file");
        file.write_all(contents.as_bytes()).expect("Failed to write to file");
        fs::remove_file(file_path).expect("Failed to remove file");
    }
}


fn create_password_file(file_path: &String) {
    let path = Path::new(&file_path);
    let display = path.display();

    // Create the 'passwords' directory if it doesn't exist
    if let Err(_) = fs::create_dir_all(path.parent().unwrap()) {
        println!("Failed to create the 'passwords' directory.");
        return;
    }

    match File::create(&path) {
        Err(why) => match why.kind() {
            ErrorKind::AlreadyExists => {
                println!("A password file already exists with that name!");
                return;
            }
            other_error => {
                panic!("couldn't create {}: {}", display, other_error)
            }
        },
        Ok(_) => println!("Password file created successfully at {}", display),
    };
}

