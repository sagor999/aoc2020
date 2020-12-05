use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

struct PasswordData {
  first_position: u16,
  second_position: u16,
  letter: char,
  password: String
}

//example: 1-9 x: xwjgxtmrzxzmkx
impl FromStr for PasswordData {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let str_parts: Vec<&str> = s.split(' ').collect();
    let occ = str_parts[0];
    let letter = str_parts[1];
    let pass = str_parts[2];

    let occ_parts: Vec<&str> = occ.split('-').collect();
    let min_occ_fromstr = occ_parts[0].parse::<u16>()?;
    let max_occ_fromstr = occ_parts[1].parse::<u16>()?;

    let letter_fromstr = letter.trim_end_matches(':');
    
    Ok(PasswordData { first_position: min_occ_fromstr, second_position: max_occ_fromstr, letter: letter_fromstr.chars().nth(0).unwrap(), password: pass.to_string()  })
  }
}

impl fmt::Display for PasswordData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}-{} {}: {}", self.first_position, self.second_position, self.letter, self.password)
  }
}

fn read<R: Read>(io: R) -> Result<Vec<PasswordData>, Error> {
  let br = BufReader::new(io);
  br.lines()
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect()
}

fn main() -> Result<(), Error> {
  let vec = read(File::open("input.txt")?)?;
  let len = vec.len();

  let mut num_valid = 0;
  for x in 0..len {
    let val_x = &vec[x];

    let first_match = val_x.password.chars().nth(usize::from(val_x.first_position-1)).unwrap() == val_x.letter;
    let second_match = val_x.password.chars().nth(usize::from(val_x.second_position-1)).unwrap() == val_x.letter;
    
    if !((first_match && second_match) || (!first_match && !second_match)) {
      num_valid = num_valid + 1;
    }
    

  }

  println!("valid passwords: {}", num_valid);

  Ok(())
}
