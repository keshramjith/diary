// use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn print_commands() {
  println!("Would you like to: ");
  println!("\t1) View all notes");
  println!("\t2) Add a new note");
  println!("\tq) To close application");

}

fn main() {
  // let f = File::open("./notes.txt").unwrap();
  // let reader = BufReader::new(f);

  let stdin = io::stdin();
  let mut buffer;
  loop {
    buffer = String::new();
    print_commands();
    stdin.lock().read_line(&mut buffer);
    match buffer.trim_end() {
      "1" => println!("View all notes!"),
      "2" => println!("Add new note!"),
      "q" => {
        println!("Quitting application!");
        break;
      },
      _ => println!("Unrecognized command, please try again.")
    }
  }
}