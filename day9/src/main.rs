use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
  let br = BufReader::new(io);
  br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

fn main() -> Result<(), Error> {
  let vec = read(File::open("input.txt")?)?;

  let mut target_number = 0;
  for i in 25..vec.len() {
    let cur_check = vec[i];
    let mut found = false;
    for x in (i-25)..i {
      let val_x = vec[x];
      for y in (x+1)..i {
        let val_y = vec[y];
        if (val_x+val_y)==cur_check {
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
    if !found {
      println!("found: {}", cur_check);
      target_number = cur_check;
      break;
    }
  }

  // second part
  for x in 0..vec.len() {
    let mut sum: u64 = vec[x];
    let mut found = false;
    for y in (x+1)..vec.len() {
      sum = sum + vec[y];
      if sum == target_number {
        println!("found: start: {}, end: {}", vec[x], vec[y]);
        let mut min = vec[x];
        let mut max = vec[x];
        for z in x..y {
          if min > vec[z] {
            min = vec[z];
          }
          if max < vec[z] {
            max = vec[z];
          }
        }
        println!("min: {}, max: {}, answer: {}", min, max, min+max);
        found = true;
        break;
      } else if sum > target_number {
        break;
      }
    }
    if found {
      break;
    }

  }

  Ok(())
}
