use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
  let br = BufReader::new(io);
  br.lines()
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect()
}

fn main() -> Result<(), Error> {
  let vec = read(File::open("input.txt")?)?;
  let len = vec.len();

  let mut cur_x = 3;
  let mut num_trees = 0;

  for y in 1..len {
    let cur_line = &vec[y];
    if cur_line.chars().nth(cur_x).unwrap() == '#' {
      num_trees = num_trees + 1;
    }
    //println!("{}", cur_line);
    cur_x = (cur_x+3)%cur_line.len();
  }

  println!("num trees: {}", num_trees);

  Ok(())
}
