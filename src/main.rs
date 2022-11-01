use std::fs::{File};
use std::io::{self, BufRead, BufReader};

fn print_commands() {
  println!("Would you like to: ");
  println!("\t1) View all notes");
  println!("\t2) Add a new note");
  println!("\tq) To close application");

}

fn view_notes<T: BufRead>(reader: T) {
  for line_ in reader.lines() {
    let line = line_.unwrap();
    println!("{}", line);
  }
}

fn main() {
  let f = File::create("notes.txt").unwrap(); // write-only, need to think around how to do reading or writing
  let mut reader;

  let stdin = io::stdin();
  let mut buffer;
  loop {
    buffer = String::new();
    reader = BufReader::new(&f);
    print_commands();
    stdin.lock().read_line(&mut buffer).unwrap();
    match buffer.trim_end() {
      "1" => view_notes(reader),
      "2" => println!("Add new note!"),
      "q" => {
        println!("Closing diary.");
        break;
      },
      _ => println!("Unrecognized command, please try again.")
    }
    println!("\n");
  }
}