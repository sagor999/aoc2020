use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
  let br = BufReader::new(io);
  br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

fn main() -> Result<(), Error> {
  let mut vec = read(File::open("input.txt")?)?;
  vec.sort();

  let mut cur_jolt = 0;
  let mut diff1 = 0;
  let mut diff2 = 0;
  let mut diff3 = 0;

  for i in 0..vec.len() {
      let diff = vec[i] - cur_jolt;
      if diff > 3 || diff == 0 {
          panic!("diff is: {}", diff);
      }
      match diff {
          1 => diff1 = diff1+1,
          2 => diff2 = diff2+1,
          3 => diff3 = diff3+1,
          _ => panic!("!"),
      };

      cur_jolt = cur_jolt + diff;
  }
  diff3 = diff3+1;
  cur_jolt = cur_jolt+3;

  println!("cur_jolt: {}, diff1: {}, diff2: {}, diff3: {}", cur_jolt, diff1, diff2, diff3);
  println!("answer: {}", diff1*diff3);

  // part 2
  let mut cache = Vec::<u64>::new();
  cache.resize(200, 0);
  cache[2] = 1;
  for i in 0..vec.len() {
    let v = vec[i] as usize;
    cache[v+2] = cache[v-1] + cache[v] + cache[v+1];
  }
  let last_elem = vec[vec.len()-1] as usize;
  println!("answer: {}", cache[last_elem+2]);

  Ok(())
}
