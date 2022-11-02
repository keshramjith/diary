use std::fs::{OpenOptions};
use std::io::{self, BufRead, BufReader, Stdin, Write};

fn print_commands() {
  println!("Would you like to: ");
  println!("\t1) View all notes");
  println!("\t2) Add a new note");
  println!("\tq) To close application");

}

fn view_notes() {
  let f = OpenOptions::new().read(true).open("notes.txt");
  match f {
    Ok(file) => {
      let reader = BufReader::new(file);
      for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{}", line);
      }
    },
    Err(_) => println!("notes.txt does not exist! Please add a note!")
  }
}

fn create_note(stdin: &Stdin) {
  let mut note_buffer = String::new();
  stdin.lock().read_line(&mut note_buffer).unwrap();
  let mut f = OpenOptions::new().append(true).write(true).create(true).open("notes.txt").unwrap();
  writeln!(&mut f, "{}", note_buffer).unwrap();
}

fn main() {
  let stdin = io::stdin();
  let mut option_buffer;
  loop {
    option_buffer = String::new();
    print_commands();
    stdin.lock().read_line(&mut option_buffer).unwrap();
    match option_buffer.trim_end() {
      "1" => view_notes(),
      "2" => {
        println!("Enter text to add to the note:");
        create_note(&stdin);
    },
      "q" => {
        println!("Closing diary.");
        break;
      },
      _ => println!("Unrecognized command, please try again.")
    }
    println!("\n");
  }
}