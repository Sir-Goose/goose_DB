use clap::*;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use bincode;
use serde::{Serialize, Deserialize};






#[derive(Serialize, Deserialize)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {map: HashMap::new() }
    }

    fn add_new_value(&mut self, key: String, value: String) -> Result<(), Error> {
        if self.map.contains_key(&key) {
            Err(Error::ValueAlreadyExists)
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }

    fn change_value(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get_value(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn remove_value(&mut self, key: &str) {
        self.map.remove(key);
    }
}

#[derive(Debug)]
enum Error {
    ValueAlreadyExists,
}


fn main() {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if there are any command line arguments.
    if args.len() <= 0 {
        println!("Usage: {} <numbers>", args[0]);
        return;
    }

    // Create an array to store the command line arguments.
    let mut array: Vec<i32> = Vec::new();

    // Iterate over the command line arguments and add them to the array.
    for arg in args[1..].iter() {
        let number: i32 = arg.parse().unwrap();
        array.push(number);
    }

    // Print the array.
    println!("{:?}", array);
    startup_check();

    let mut db = Database::new();
    db.add_new_value("key1".to_string(), "value1".to_string()).unwrap();
    db.add_new_value("key2".to_string(), "value2".to_string()).unwrap();
    db.add_new_value("key1".to_string(), "value3".to_string()).unwrap_err();
    println!("{:?}", db.get_value("key1")); // Some("value1")
    db.change_value("key1".to_string(), "value4".to_string());
    println!("{:?}", db.get_value("key1")); // Some("value4")
    db.remove_value("key2");
    println!("{:?}", db.get_value("key2")); // None


    //println!("Argument 1: {} Argument 2: {} Argument 3: {}", arguments.arg_1, arguments.arg_2, arguments.arg_3);
    //decision_tree(arguments.arg_1, arguments.arg_2, arguments.arg_3)

}
fn decision_tree(database_operation : String, key : String, value : String)  {
    match database_operation.as_str() {
        "read" => read_database(key, value),
        "write" => write_database(key, value),
        "modify" => modify_database(key, value),
        "delete" => delete_from_database(key, value),
        _ => println!("{} is not a valid operation.", database_operation),
    }
}

fn startup_check() {
    // check if database exists
    // if not create database

    if let Err(err) = load_from_disk() {
        eprintln!("Failed to load from disk: {}", err);
        // Handle the error appropriately (e.g., create a new database or exit).
    }

}

fn read_database(key : String, value : String) {
    println!("Reading data from the database");
}

fn write_database(key : String, value : String) {
    println!("Writing to the database");
}

fn modify_database(key : String, value : String) {
    println!("Modifying data in the database");
}

fn delete_from_database(key : String, value : String) {
    println!("Deleting data from the database");
}
fn save_to_disk(database: &Database) -> Result<(), Box<dyn std::error::Error>> {
    /*
future self use this when calling save_to_disk to handle the error

 if let Err(err) = save_to_disk(&your_database) {
    eprintln!("Failed to save to disk: {}", err);
    // Handle the error appropriately (e.g., exit the program or retry).
}

 */
    // Serialize the database to JSON or any other desired format
    let serialized = serde_json::to_string(database)?;

    // Create a new file.
    let mut file = std::fs::File::create("database.json")?;

    // Write the serialized data to the file.
    file.write_all(serialized.as_bytes())?;

    Ok(())

}

fn load_from_disk() -> Result<Database, Box<dyn std::error::Error>> {
    /*
    match load_from_disk() {
    Ok(database) => {
        // Successfully loaded the database, use it as needed.
        // For example, assign it to a variable or work with it directly.
        let your_database = database;
    }
    Err(err) => {
        eprintln!("Failed to load from disk: {}", err);
        // Handle the error appropriately (e.g., create a new database or exit).
    }
}

     */


    // Open the file for reading.
    let file = std::fs::File::open("database.json");

    match file {
        Ok(file) => {
            // Create a buffered reader for efficient reading.
            let reader = std::io::BufReader::new(file);

            // Deserialize the data from the file.
            match serde_json::from_reader(reader) {
                Ok(database) => Ok(database),
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}







