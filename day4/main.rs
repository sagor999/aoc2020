use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;


fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut collected_fields = HashSet::<String>::new();
  let mut num_valid_password = 0;

  for line in lines_iter {
    let line_len = line.len();
    if line_len == 0 {
      if collected_fields.len() == 7 {
        num_valid_password = num_valid_password+1;
      }
      collected_fields.clear();
    } else {
      let entries: Vec<&str> = line.split(' ').collect();
      for entry in entries {
        let fields: Vec<&str> = entry.split(':').collect();
        let field = fields[0];
        let is_valid_field = match field {
          "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => true,
          _ => false,
        };
        if is_valid_field {
          collected_fields.insert(field.to_owned());
        }
      }      
    }
    if line.len() == 0 {
      println!("valid passwords so far: {}", num_valid_password);  
    } else {
      println!("{}, {}", line, collected_fields.len());
    }
  }
  if collected_fields.len() == 7 {
    num_valid_password = num_valid_password+1;
  }
  println!("valid passports: {}", num_valid_password);
  
  Ok(())
}
