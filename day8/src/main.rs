use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone)]
struct Instruction {
  inst: String,
  offs: i16,
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {:+}", self.inst, self.offs)
  }
}

fn process_line(s: &str) -> Instruction {
  // example: acc -99
  let (inst, offset_str) = s.split_at(s.find(" ").unwrap());
  let offset_str_trim = offset_str.trim();
  let offset: i16 = offset_str_trim.parse::<i16>().unwrap();

  let instruction = Instruction {
    inst: inst.to_string(),
    offs: offset,
  };

  return instruction;
}

// runs through program.
fn run_program(instructions: &Vec<Instruction>) -> (bool, i16, Vec<usize>) {
  let mut did_finish_correctly = false;
  let mut acc: i16 = 0;

  let mut processed_line = HashSet::<usize>::new();
  let mut cur_inst_index = 0;
  let mut stack = Vec::<usize>::new();
  loop {
    if processed_line.contains(&cur_inst_index) {
      break;
    }
    if cur_inst_index == instructions.len() {
      did_finish_correctly = true;
      break;
    }
    stack.push(cur_inst_index);
    let inst = &instructions[cur_inst_index];
    processed_line.insert(cur_inst_index);
    match inst.inst.as_str() {
      "nop" => cur_inst_index = cur_inst_index + 1,
      "acc" => {
        acc = acc + inst.offs;
        cur_inst_index = cur_inst_index + 1
      }
      "jmp" => cur_inst_index = ((cur_inst_index as i16) + inst.offs) as usize,
      _ => panic!("found unexpected instruction"),
    }
  }
  return (did_finish_correctly, acc, stack);
}

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut instructions = Vec::<Instruction>::new();

  for line in lines_iter {
    //println!("processing line: {}", line);
    instructions.push(process_line(&line));
  }

  let (_, acc, stack) = run_program(&instructions);
  println!("first part: acc: {}", acc);

  // second part of the problem
  // go back through stack, and try to modify one instr at a time until program will finish correctly
  for stack_val in stack.iter().rev() {
    let i = *stack_val;
    let mut instr_copy = instructions.to_vec();
    println!("trying instruction at line {}: {}", i, instr_copy[i]);
    if instr_copy[i].inst == "jmp" {
      instr_copy[i].inst = "nop".to_string();
    } else if instr_copy[i].inst == "nop" {
      instr_copy[i].inst = "jmp".to_string();
    } else {
      continue;
    }
    let (did_finish, new_acc, _) = run_program(&instr_copy);
    if did_finish {
      println!("finished! acc: {}", new_acc);
      break;
    }
  }

  Ok(())
}
