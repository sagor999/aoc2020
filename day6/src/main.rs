use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//#[macro_use]
//extern crate more_asserts;

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut collected_answers = HashSet::<char>::new();
  let mut total_answered_questions = 0;
  let mut total_everyone_answered_same_question = 0;
  let mut collected_yes_answers = Vec::<char>::new();
  let mut first_group = true;

  for line in lines_iter {
    //println!("processing line: {}", line);

    if line.len() == 0 {
      // new group
      //println!("total: {}", collected_answers.len());
      total_answered_questions = total_answered_questions + collected_answers.len();
      collected_answers.clear();

      //println!("total yes: {}", collected_yes_answers.len());
      total_everyone_answered_same_question =
        total_everyone_answered_same_question + collected_yes_answers.len();
      collected_yes_answers.clear();
      first_group = true;
    } else {
      for ch in line.chars() {
        collected_answers.insert(ch);
      }
      // sort line to make it easier and faster to work with it
      let mut chars: Vec<char> = line.chars().collect();
      chars.sort_by(|a, b| a.cmp(b));

      if first_group {
        collected_yes_answers = chars;
        first_group = false;
      } else {
        let mut union_anwers = Vec::<char>::new();
        //println!("line 1: {:?}", collected_yes_answers);
        //println!("line 2: {:?}", chars);
        let mut i = 0;
        let mut j = 0;
        loop {
          if i == collected_yes_answers.len() || j == chars.len() {
            break;
          }
          let first_char = collected_yes_answers[i];
          let second_char = chars[j];
          if first_char == second_char {
            union_anwers.push(first_char);
            i = i + 1;
            j = j + 1;
          } else {
            if first_char < second_char {
              i = i + 1;
            } else {
              j = j + 1;
            }
          }
        }
        //println!("union: {:?}", union_anwers);
        collected_yes_answers = union_anwers;
      }
    }
  }
  // collect final result
  total_answered_questions = total_answered_questions + collected_answers.len();
  total_everyone_answered_same_question =
    total_everyone_answered_same_question + collected_yes_answers.len();

  println!("total collected answers: {}", total_answered_questions);
  println!(
    "total answers where everyones answered yes: {}",
    total_everyone_answered_same_question
  );
  Ok(())
}
