use clap::*;


#[derive(Parser)]
struct Arguments {
    arg_1 : String,
    arg_2 : String,
    arg_3 : String
}

fn main() {
    let arguments : Arguments = Arguments::parse();

    println!("Argument 1: {} Argument 2: {} Argument 3: {}", arguments.arg_1, arguments.arg_2, arguments.arg_3)

}
    fn decision_tree() {

    }




