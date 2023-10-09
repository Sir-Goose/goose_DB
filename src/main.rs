use clap::*;
use std::collections::HashMap;
use bincode;



#[derive(Parser)]
struct Arguments {
    /// Read, Write, Modify, Delete
    arg_1 : String,
    /// Key
    arg_2 : String,
    /// Pair
    arg_3 : String
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {map: HashMap::new() }
    }

    fn add(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get_value(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

}

fn main() {
    let arguments : Arguments = Arguments::parse();
    startup_check();


    println!("Argument 1: {} Argument 2: {} Argument 3: {}", arguments.arg_1, arguments.arg_2, arguments.arg_3);
    decision_tree(arguments.arg_1, arguments.arg_2, arguments.arg_3)

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







