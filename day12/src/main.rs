use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn calc_new_dir(x: i32, y: i32, cmd: &str, val: i32) -> (i32, i32) {
  let deg = val as f32;
  let mut rad = deg.to_radians();
  if cmd == "R" {
    rad = -rad;
  }
  let cs = rad.cos();
  let sn = rad.sin();

  let xf = x as f32;
  let yf = y as f32;

  let px = (xf * cs - yf * sn).round() as i32;
  let py = (xf * sn + yf * cs).round() as i32;

  return (px, py);
}

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut pos_x = 0;
  let mut pos_y = 0;
  let mut dir_x = 1;
  let mut dir_y = 0;
  for line in lines_iter {
    let (cmd, value_str) = line.split_at(1);
    let value = value_str.parse::<i32>().unwrap();
    match cmd {
      "N" => pos_y = pos_y + value,
      "S" => pos_y = pos_y - value,
      "E" => pos_x = pos_x + value,
      "W" => pos_x = pos_x - value,
      "F" => {
        pos_x = pos_x + dir_x*value;
        pos_y = pos_y + dir_y*value;
      }
      "L" => {
        let (nx, ny) = calc_new_dir(dir_x, dir_y, cmd, value);
        dir_x = nx;
        dir_y = ny;
      }
      "R" => {
        let (nx, ny) = calc_new_dir(dir_x, dir_y, cmd, value);
        dir_x = nx;
        dir_y = ny;
      }
      _ => panic!("unexpected cmd"),
    }
  }

  println!("result: {}", pos_x.abs() + pos_y.abs());

  // part 2
  let br2 = BufReader::new(File::open("input.txt")?);
  let lines_iter2 = br2.lines().map(|l| l.unwrap());

  pos_x = 0;
  pos_y = 0;
  let mut w_x = 10;
  let mut w_y = 1;
  for line in lines_iter2{
    let (cmd, value_str) = line.split_at(1);
    let value = value_str.parse::<i32>().unwrap();
    match cmd {
      "N" => w_y = w_y + value,
      "S" => w_y = w_y - value,
      "E" => w_x = w_x + value,
      "W" => w_x = w_x - value,
      "F" => {
        pos_x = pos_x + w_x*value;
        pos_y = pos_y + w_y*value;
      }
      "L" => {
        let (nx, ny) = calc_new_dir(w_x, w_y, cmd, value);
        w_x = nx;
        w_y = ny;
      }
      "R" => {
        let (nx, ny) = calc_new_dir(w_x, w_y, cmd, value);
        w_x = nx;
        w_y = ny;
      }
      _ => panic!("unexpected cmd"),
    }
  }

  println!("result: {}", pos_x.abs() + pos_y.abs());

  Ok(())
}
