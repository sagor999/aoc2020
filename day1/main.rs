use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
  let br = BufReader::new(io);
  br.lines()
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect()
}

fn main() -> Result<(), Error> {
  let vec = read(File::open("input.txt")?)?;
  let len = vec.len();

  for x in 0..len - 1 {
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
  }
 
  Ok(())
}
