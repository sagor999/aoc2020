use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
  let br = BufReader::new(io);
  br.lines()
      .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
      .collect()
}

fn traverse(x_incr: usize, y_incr: usize, map: &Vec<String>) -> i32 {
  let len = map.len();

  let mut cur_x = x_incr;
  let mut num_trees = 0;

  for y in (y_incr..len).step_by(y_incr) {
    let cur_line = &map[y];
    if cur_line.chars().nth(cur_x).unwrap() == '#' {
      num_trees = num_trees + 1;
    }
    //println!("{}", cur_line);
    cur_x = (cur_x+x_incr)%cur_line.len();
  }
  return num_trees
}

fn main() -> Result<(), Error> {
  let vec = read(File::open("input.txt")?)?;
  
  println!("1,1: {}", traverse(1, 1, &vec));
  println!("3,1: {}", traverse(3, 1, &vec));
  println!("5,1: {}", traverse(5, 1, &vec));
  println!("7,1: {}", traverse(7, 1, &vec));
  println!("1,2: {}", traverse(1, 2, &vec));

  Ok(())
}
