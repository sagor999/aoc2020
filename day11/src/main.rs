use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::fmt;
use std::cmp;

#[derive(Clone, Copy, PartialEq)]
enum SeatState {
  Floor,
  Empty,
  Taken,
}

impl fmt::Display for SeatState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SeatState::Floor => write!(f, "."),
      SeatState::Empty => write!(f, "L"),
      SeatState::Taken => write!(f, "#"),
    }
  }
}


fn print_seat_map(map: &Vec<Vec<SeatState>>) {
  for col in 0..map.len() {
    for row in 0..map[col].len() {
      print!("{}", map[col][row])
    }
    println!("");
  }
}

fn num_taken_adj_seats(seats: &Vec<Vec<SeatState>>, col: usize, row: usize) -> u32 {
  let mut num_adj_taken = 0;
  let col_start = cmp::max((col as i16)-1, 0) as usize;
  let col_end = cmp::min(col+1, seats.len()-1);
  let row_start = cmp::max((row as i16)-1, 0) as usize;
  let row_end = cmp::min(row+1, seats[0].len()-1);

  for c in col_start..col_end+1 {
    for r in row_start..row_end+1 {
      if c == col && r == row {
        continue;
      }
      if seats[c][r] == SeatState::Taken {
        num_adj_taken = num_adj_taken+1;
      }
    }
  }

  return num_adj_taken
}

fn num_vis_taken_adj_seats(seats: &Vec<Vec<SeatState>>, col: usize, row: usize) -> u32 {
  let mut num_adj_taken = 0;

  // check 8 directions
  let dirs = vec![(-1,0),(-1,-1),(0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1)];
  for dir in dirs {
    let (xd, yd) = dir;
    let mut x = (col as i32)+xd;
    let mut y = (row as i32)+yd;
    loop {
      if x < 0 || y < 0 || x>=(seats.len() as i32) || y >=(seats[0].len() as i32) {
        break;
      }
      if seats[x as usize][y as usize] != SeatState::Floor {
        if seats[x as usize][y as usize] == SeatState::Taken {
          num_adj_taken = num_adj_taken+1;
        }
        break;
      }
      x = x+xd;
      y = y+yd;
    }
  }

  return num_adj_taken
}

fn tick(seats: &Vec<Vec<SeatState>>) -> (Vec<Vec<SeatState>>, u32) {
  let mut new_seats = vec![vec![SeatState::Empty;0];seats.len()];
  let mut num_changes = 0;

  for c in 0..seats.len() {
    for r in 0..seats[c].len() {
      let adj_taken = num_taken_adj_seats(seats, c, r);
      if seats[c][r] == SeatState::Empty && adj_taken == 0 {
        new_seats[c].push(SeatState::Taken);
        num_changes = num_changes+1;
      } else if seats[c][r] == SeatState::Taken && adj_taken >= 4 {
        new_seats[c].push(SeatState::Empty);
        num_changes = num_changes+1;
      } else {
        new_seats[c].push(seats[c][r]);
      }
    }
  }

  return (new_seats,num_changes)
}

fn tick_vis(seats: &Vec<Vec<SeatState>>) -> (Vec<Vec<SeatState>>, u32) {
  let mut new_seats = vec![vec![SeatState::Empty;0];seats.len()];
  let mut num_changes = 0;

  for c in 0..seats.len() {
    for r in 0..seats[c].len() {
      let adj_taken = num_vis_taken_adj_seats(seats, c, r);
      if seats[c][r] == SeatState::Empty && adj_taken == 0 {
        new_seats[c].push(SeatState::Taken);
        num_changes = num_changes+1;
      } else if seats[c][r] == SeatState::Taken && adj_taken >= 5 {
        new_seats[c].push(SeatState::Empty);
        num_changes = num_changes+1;
      } else {
        new_seats[c].push(seats[c][r]);
      }
    }
  }

  return (new_seats,num_changes)
}

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let col_len = 98;

  let mut seat_map = vec![vec![SeatState::Empty;0];col_len];
  let mut cur_col = 0;
  for line in lines_iter {
    for ch in line.chars() {
      match ch {
        'L' => seat_map[cur_col].push(SeatState::Empty),
        '.' => seat_map[cur_col].push(SeatState::Floor),
        _ => panic!("unknown symbol!"),
      }
    }
    cur_col = cur_col+1;
  }

  //print_seat_map(&seat_map);
  loop {
    let (new_seat_map, num_changes) = tick_vis(&seat_map);
    seat_map = new_seat_map;
    println!("{}", num_changes);
    if num_changes == 0 {
      break;
    }
  }

  print_seat_map(&seat_map);

  let mut num_occ = 0;
  for c in 0..seat_map.len() {
    for r in 0..seat_map[c].len() {
      if seat_map[c][r] == SeatState::Taken {
        num_occ = num_occ + 1;
      }
    }
  }

  println!("answer: {}", num_occ);
  
  Ok(())
}
