use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[macro_use] 
extern crate more_asserts;

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut highest_seat_id = 0;

  let mut all_seats = vec![false; 1024];

  for line in lines_iter {
    //println!("{}", line);
    let mut row_start = 0;
    let mut row_end = 127;
    let mut col_start = 0;
    let mut col_end = 7;

    for ch in line.chars() {
      let row_range_half = ((row_end-row_start)+1)/2;
      let col_range_half = ((col_end-col_start)+1)/2;
      if ch == 'F' {
        // take lower half of row range
        row_end = row_end - row_range_half;
        assert_ge!(row_end, row_start);
      } else if ch == 'B' {
        // take upper half of row range
        row_start = row_start + row_range_half;
        assert_le!(row_start, row_end);
      } else if ch == 'R' {
        // take upper half of col range
        col_start = col_start + col_range_half;
        assert_le!(col_start, col_end);
      } else if ch == 'L' {
        // take lower half of col range
        col_end = col_end - col_range_half;
        assert_ge!(col_end, col_start);
      }
    }
    assert_eq!(row_start, row_end);
    assert_eq!(col_start, col_end);
    let seat_id = row_start*8+col_start;
    assert_ge!(seat_id, 0);
    assert_lt!(seat_id, 1024);
    all_seats[seat_id] = true;
    println!("ticket: {}, row: {}, col: {}, seatID: {}", line, row_start, col_start, seat_id);
    if seat_id > highest_seat_id {
      highest_seat_id = seat_id;
    }
  }

  println!("result: {}", highest_seat_id);

  for ind in 1..1023 {
    let prev_seat = all_seats[ind-1];
    let next_seat = all_seats[ind+1];
    if all_seats[ind]==false && prev_seat==true && next_seat==true {
      println!("My seat ID is: {}", ind);
    }
  }
  
  Ok(())
}
