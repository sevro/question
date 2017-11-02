use std::collections::HashMap;
use std::io::{self, BufRead, Write};

pub struct Question {
    question: String,
    prompt: String,
    default: Option<Answer>,
    acceptable: Option<Vec<String>>,
    valid_responses: Option<HashMap<String, Answer>>,
    clarification: Option<String>,
    tries: Option<u64>,
    until_acceptable: bool,
    show_defaults: bool,
    yes_no: bool,
}

impl Question {
    pub fn new(question: &str) -> Question {
        let question = question.to_string();
        Question {
            question: question.clone(),
            prompt: question,
            default: None,
            acceptable: None,
            valid_responses: None,
            clarification: None,
            tries: None,
            until_acceptable: false,
            show_defaults: false,
            yes_no: false,
        }
    }

    pub fn accept<'a>(&'a mut self, accepted: &str) -> &'a mut Question {
        let accepted = accepted.to_string();
        match self.acceptable {
            Some(ref mut vec) => vec.push(accepted),
            None => {
                let mut vec = Vec::new();
                vec.push(accepted);
                self.acceptable = Some(vec);
            },
        }
        self
    }

    pub fn acceptable<'a>(&'a mut self, accepted: &[String]) -> &'a mut Question {
        match self.acceptable {
            Some(ref mut vec) => vec.append(&mut accepted.to_vec()),
            None => {
                let vec = accepted.to_vec();
                self.acceptable = Some(vec);
            },
        }
        self
    }

    /// Shorhand for yes("yes") yes("y") no("no") no("n")
    pub fn yes_no<'a>(&'a mut self) -> &'a mut Question {
        self.yes_no = true;
        let response_keys = vec![
                                String::from("yes"),
                                String::from("y"),
                                String::from("no"),
                                String::from("n")
                            ];

        let response_values = vec![Answer::YES, Answer::YES, Answer::NO, Answer::NO];
        let mut valid_responses: HashMap<String, Answer> = response_keys.into_iter()
            .zip(response_values.into_iter())
            .collect();

        match self.valid_responses {
            Some(ref mut hashmap) => {
                for (k, v) in valid_responses.drain() {
                    hashmap.insert(k, v);
                }
            },
            None => self.valid_responses = Some(valid_responses),
        }
        self
    }

    pub fn tries<'a>(&'a mut self, tries: u64) -> &'a mut Question {
        match tries {
            0 => self.until_acceptable = true,
            1 => return self,
            _ => self.tries = Some(tries),
        }
        self
    }

    pub fn until_acceptable<'a>(&'a mut self) -> &'a mut Question {
        self.until_acceptable = true;
        self
    }

    pub fn show_defaults<'a>(&'a mut self) -> &'a mut Question {
        self.show_defaults = true;
        self
    }

    pub fn default<'a>(&'a mut self, answer: Answer) -> &'a mut Question {
        self.default = Some(answer);
        self
    }

    pub fn ask<'a>(&mut self) -> Option<Answer> {
        if self.yes_no {
            self.build_prompt();
        }
        let prompt = self.prompt.clone();
        let mut tries = 0;
        let valid_responses = self.valid_responses.clone().unwrap();
        loop {
            let stdio = io::stdin();
            let input = stdio.lock();
            let output = io::stdout();
            if let Ok(response) = prompt_user(input, output, &prompt) {
                for key in valid_responses.keys() {
                    if *response.trim().to_lowercase() == *key {
                        return Some(valid_responses.get(key).unwrap().clone());
                    }
                }
                if !self.until_acceptable {
                    match self.tries {
                        None => return None,
                        Some(max_tries) if tries >= max_tries => return None,
                        Some(_) => tries += 1,
                    }
                }
                self.build_clarification();
            }
        }
    }

    /// Shorthand for yes_no() until_acceptable()
    pub fn confirm(&mut self) -> Answer {
        self.yes_no();
        self.build_prompt();
        let prompt = self.prompt.clone();
        let valid_responses = self.valid_responses.clone().unwrap();
        loop {
            let stdio = io::stdin();
            let input = stdio.lock();
            let output = io::stdout();
            if let Ok(response) = prompt_user(input, output, &prompt) {
                for key in valid_responses.keys() {
                    if *response.trim().to_lowercase() == *key {
                        return valid_responses.get(key).unwrap().clone();
                    }
                }
                self.build_clarification();
            }
        }
    }

    fn build_prompt(&mut self) {
        if self.show_defaults {
            match self.default {
                Some(Answer::YES) => self.prompt += "[Y/n]",
                Some(Answer::NO) => self.prompt += "[y/N]",
                None => self.prompt += "[y/n]",
                Some(_) => panic!(),
            }
        }
    }

    fn build_clarification(&mut self) {
        if let Some(clarification) = self.clarification.clone() {
            self.prompt = clarification;
            self.prompt += "\n";
            self.prompt += &self.question;
            self.build_prompt();
        }
    }
}

#[cfg(not(test))]
fn prompt_user<R, W>(mut reader: R, mut writer: W, question: &str) -> Result<String, std::io::Error>
    where R: BufRead,
          W: Write
{
    write!(&mut writer, "{}", question)?;
    let mut s = String::new();
    reader.read_line(&mut s)?;
    Ok(s)
}

#[cfg(test)]
fn prompt_user<R, W>(mut reader: R, mut writer: W, question: &str) -> Result<String, std::io::Error>
    where R: BufRead,
          W: Write
{
    use tests;
    Ok( unsafe { tests::test_response.clone().to_string() } )
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Answer {
    RESPONSE(String),
    YES,
    NO,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    const QUESTION: &'static str = "what is the meaning to life, the universe, and everything";
    const ANSWER: &'static str = "42";
    pub static mut test_response: &str = "";

    /*
    #[test]
    fn prompt() {
        let input = Cursor::new(&b"42"[..]);
        let mut output = Cursor::new(Vec::new());
        let answer = prompt_user(input, &mut output, QUESTION).unwrap();
        let output = String::from_utf8(output.into_inner()).expect("Not UTF-8");
        assert_eq!(QUESTION, output);
        assert_eq!(ANSWER, answer);
    }
    */

    #[test]
    fn simple_confirm() {
        unsafe { test_response = "y" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::YES, answer);

        unsafe { test_response = "Y" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::YES, answer);

        unsafe { test_response = "yes" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::YES, answer);

        unsafe { test_response = "YES" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::YES, answer);

        unsafe { test_response = "yES" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::YES, answer);

        unsafe { test_response = "n" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::NO, answer);

        unsafe { test_response = "N" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::NO, answer);

        unsafe { test_response = "no" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::NO, answer);

        unsafe { test_response = "NO" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::NO, answer);

        unsafe { test_response = "nO" };
        let answer = Question::new("Blue").confirm();
        assert_eq!(Answer::NO, answer);
    }
}