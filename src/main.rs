use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  println!("Would you like to: ");
  println!("\t1) View all notes");
  println!("\t2) Add a new note");

  let f = File::open("./notes.txt").unwrap();

  let stdin = io::stdin();
  let word = stdin.lock();
  let worden = word.lines();
  for line_ in worden {
    let line = line_.unwrap();
    println!("{}", line)
  }
}