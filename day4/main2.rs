use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;
use regex::Regex;

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut collected_fields = HashSet::<String>::new();
  let mut num_valid_password = 0;

  let re_hgt = Regex::new(r"(^(59|(6\d{1})|(7[1-6]))in$|((^1([5-8])\d)|^190|^191|^192|^193)cm$)").unwrap();
  let re_hcl = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
  let re_pid = Regex::new(r"^\d{9}$").unwrap();

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
        let value = fields[1];
        assert_eq!(fields.len(), 2);
        let is_valid_field = match field {
          "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => true,
          _ => false,
        };
        if is_valid_field {
          // now lets do some field validation
          let mut is_valid = false;
          if field == "byr" {
            let byr = value.parse::<i32>().unwrap();
            if byr >= 1920 && byr <= 2002 {
              is_valid = true;
            }
          } else if field == "iyr" {
            let iyr = value.parse::<i32>().unwrap();
            if iyr >= 2010 && iyr <= 2020 {
              is_valid = true;
            }
          } else if field == "eyr" {
            let eyr = value.parse::<i32>().unwrap();
            if eyr >= 2020 && eyr <= 2030 {
              is_valid = true;
            }
            //println!("{}, {}, {}, {}", field, value, eyr, is_valid);
          } else if field == "hgt" {
            is_valid = re_hgt.is_match(value);
          } else if field == "hcl" {
            is_valid = re_hcl.is_match(value);
          } else if field == "ecl" {
            is_valid = match value {
              "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
              _ => false,
            };
          } else if field == "pid" {
            is_valid = re_pid.is_match(value);
          }
          
          //if is_valid == false {
          //  println!("incorrect field: {}, {}", field, value);
          //}

          //if field == "pid" {
          //  println!("pid:{} = {} [000000000]", value, is_valid);
          //}

          if is_valid {
            collected_fields.insert(field.to_owned());
          }
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
