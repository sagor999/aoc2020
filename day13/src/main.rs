use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let mut lines_iter = br.lines().map(|l| l.unwrap());

  let ts = lines_iter.next().unwrap().parse::<i32>().unwrap();
  let buses_str = lines_iter.next().unwrap();

  let buses_split: Vec<&str> = buses_str.split(',').collect();
  let mut buses = Vec::<i32>::new();
  for bus_str in buses_split {
    if bus_str == "x" {
      continue;
    }
    let bus_id = bus_str.parse::<i32>().unwrap();
    buses.push(bus_id);
  }
  buses.sort();
  let mut target_ts = ts*2;
  let mut target_bus = 0;
  println!("{:?}", buses);
  for i in 0..buses.len() {
    let bus_id = buses[i];
    let m = ts-(ts%bus_id)+bus_id;
    println!("id: {}, earliest ts: {}", bus_id, m);
    if m < target_ts {
      target_ts = m;
      target_bus = bus_id;
    }
  }
  println!("earliest ts: {}, bus_id: {}, answer: {}", target_ts, target_bus, target_bus*(target_ts-ts));

  // part2
  let buses_split: Vec<&str> = buses_str.split(',').collect();
  let mut buses = Vec::<u64>::new();
  for bus_str in buses_split {
    if bus_str == "x" {
      buses.push(1);
      continue;
    }
    let bus_id = bus_str.parse::<u64>().unwrap();
    buses.push(bus_id);
  }

  let mut target:u64 = 0;
  let mut step:u64 = 1;
  for i in 0..buses.len() {
    loop {
      if (target+(i as u64)) % buses[i] == 0 {
        break;
      }
      target = target + step;
    }
    step = step * buses[i];      
  }
  println!("answer: {}", target);

  Ok(())
}
