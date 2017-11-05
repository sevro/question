extern crate question;
use question::{Question, Answer};

fn main() {
    let default = Answer::RESPONSE(String::from("42"));
    let answer = Question::new("What is the answer to the Ultimate Question of Life, the Universe, and Everything?")
        .default(default.clone())
        .show_defaults()
        .ask()
        .unwrap();
    let correct = default;
    assert_eq!(answer, correct);
}