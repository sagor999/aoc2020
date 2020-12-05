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
    for y in 0..len2-1 {
      let val_y = second[y];
      let (_temp, third) = second.split_at(y+1);
      let len3 = third.len();
      for z in 0..len3 {
        let val_z = third[z];
        //println!("x: {}, y: {}, z: {}", val_x, val_y, val_z);

        if (val_x + val_y + val_z) == 2020 {
          let result = val_x * val_y * val_z;
          println!("x: {}, y: {}, z: {}, result: {}", val_x, val_y, val_z, result)
        }
      }
    }
  }
 
  Ok(())
}
