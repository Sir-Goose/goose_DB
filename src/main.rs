use clap::*;


#[derive(Parser)]
struct Arguments {
    /// Read, Write, Modify, Delete
    arg_1 : String,
    /// Key
    arg_2 : String,
    /// Pair
    arg_3 : String
}

fn main() {
    let arguments : Arguments = Arguments::parse();

    println!("Argument 1: {} Argument 2: {} Argument 3: {}", arguments.arg_1, arguments.arg_2, arguments.arg_3)

}
fn decision_tree(database_operation : String) {
    let read : String = "Read"


    match database_operation {
        "Read" => {
            return "read"
        }
        _ => println!("{} is not a valid operation.", database_operation),


    }


}




