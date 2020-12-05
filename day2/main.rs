use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

struct PasswordData {
  min_occurance: i32,
  max_occurance: i32,
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
    let min_occ_fromstr = occ_parts[0].parse::<i32>()?;
    let max_occ_fromstr = occ_parts[1].parse::<i32>()?;

    let letter_fromstr = letter.trim_end_matches(':');
    
    Ok(PasswordData { min_occurance: min_occ_fromstr, max_occurance: max_occ_fromstr, letter: letter_fromstr.chars().nth(0).unwrap(), password: pass.to_string()  })
  }
}

impl fmt::Display for PasswordData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}-{} {}: {}", self.min_occurance, self.max_occurance, self.letter, self.password)
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

    let mut occurances = 0;
    
    for c in val_x.password.chars() {
      if c == val_x.letter {
        occurances = occurances+1;
      }
    }

    //println!("input: {}; found occurances: {}", val_x.to_string(), occurances);
    if occurances >= val_x.min_occurance && occurances <= val_x.max_occurance {
      num_valid = num_valid + 1;
    }

  }

  println!("valid passwords: {}", num_valid);

  /*for x in 0..len - 1 {
    let (first,second) = vec.split_at(x+1);
    let len2 = second.len();
    let val_x = first[x];
    for y in 0..len2 {
      let val_y = second[y];
      //println!("x: {}, y: {}", val_x, val_y)
      if (val_x + val_y) == 2020 {
        let result = val_x * val_y;
        println!("x: {}, y: {}, result: {}", val_x, val_y, result)
      }
    }
  }*/
 
  Ok(())
}
