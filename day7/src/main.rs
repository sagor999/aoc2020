use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::fmt;



struct BagContent {
  num_bags: u32,
  bag_type: String
}

impl fmt::Display for BagContent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {}", self.num_bags, self.bag_type)
  }
}

fn process_line(s: &str) -> (String, Vec::<BagContent>) {
  // example: light green bags contain 1 pale orange bag, 3 dim fuchsia bags.
  let mut bags_found = Vec::<BagContent>::new();

  let (master_bag, all_bags) = s.split_at(s.find("contain").unwrap());
  let master_bag_trimmed = master_bag.trim_end_matches(" bags ");
  //println!("split: {}, {}", master_bag_trimmed, all_bags);
  if all_bags.find("no other bags") == None {
    let all_bags_trimmed = all_bags.trim_start_matches("contain ").trim_end_matches('.');
    let bags: Vec<&str> = all_bags_trimmed.split(',').collect();
    for bag in bags {
      let bag_trim = bag.trim().trim_end_matches(" bag").trim_end_matches(" bags");
      //println!("bag: {}", bag_trim);
      let (num_bags_str, bag_color) = bag_trim.split_at(bag_trim.find(' ').unwrap());
      let bag_color_trim = bag_color.trim();
      let num_bags: u32 = num_bags_str.parse::<u32>().unwrap();
      //println!("num: {}, color: {}", num_bags, bag_color_trim);
      bags_found.push(BagContent{ num_bags: num_bags, bag_type: bag_color_trim.to_string()});
    }
  }

  return (master_bag_trimmed.to_string(), bags_found)
}

fn does_contain_shiny_gold_bag(bags: &HashMap::<String,Vec::<BagContent>>, bag: &str) -> bool {
  let contents = &bags[bag];
  for c in contents {
    if c.bag_type == "shiny gold" || does_contain_shiny_gold_bag(bags, &c.bag_type) {
      return true;
    }
  }

  return false
}

fn count_all_bags_inside(bags: &HashMap::<String,Vec::<BagContent>>, bag: &str) -> u32 {
  let contents = &bags[bag];
  let mut num_bags = 0;
  for c in contents {
    num_bags = num_bags + c.num_bags + c.num_bags*count_all_bags_inside(bags, &c.bag_type);
  }
  return num_bags
}

fn main() -> Result<(), Error> {
  let br = BufReader::new(File::open("input.txt")?);
  let lines_iter = br.lines().map(|l| l.unwrap());

  let mut all_bags_hashmap = HashMap::<String,Vec::<BagContent>>::new();

  for line in lines_iter {
    //println!("processing line: {}", line);
    let (master_bag, all_bags) = process_line(&line);
    all_bags_hashmap.insert(master_bag, all_bags);
  }

  let mut num_shiny_gold_bags = 0;
  
  for bag_type in all_bags_hashmap.keys() {
    if does_contain_shiny_gold_bag(&all_bags_hashmap, &bag_type) {
      //println!("{} contains shiny gold bag", bag_type);
      num_shiny_gold_bags = num_shiny_gold_bags+1;
    }
  }

  println!("num shiny gold bags: {}", num_shiny_gold_bags);

  let shiny_gold_bag_content = &all_bags_hashmap["shiny gold"];
  let mut num_bags_in_shiny_gold_bag: u32 = 0;
  for bag in shiny_gold_bag_content {
    num_bags_in_shiny_gold_bag = num_bags_in_shiny_gold_bag + bag.num_bags + bag.num_bags*count_all_bags_inside(&all_bags_hashmap, &bag.bag_type);
  }

  println!("num bags inside shiny gold: {}", num_bags_in_shiny_gold_bag);

  Ok(())
}
