use std::{collections::HashMap, fs};
use regex::*;

fn find_next<'a>(nodes: &'a HashMap<&'a str, [&'a str; 2]>, input: &'a str, direction: char) -> &'a str {
    let i: usize;
    match direction {
        'L' => {
            i = 0;
        },
        'R' => {
            i = 1;
        },
        _ => unreachable!()
    }

    let x = nodes.get(&input);
    let y = &x.unwrap()[i];
    y

}

fn find_steps<'a>(nodes: &'a HashMap<&'a str, [&'a str; 2]>, directions: &'a str, mut result: &'a str, mut steps: i32) -> i32 {
    for d in directions.chars() {
        if result == "ZZZ" {
            //println!("{:?}", steps);
            return steps
        } else {
            result = find_next(&nodes, result, d);
            steps += 1;
        }
    }
    if result != "ZZZ" {
        find_steps(&nodes, directions, result, steps)
    } else if result == "ZZZ" {
        return steps
    } else {
        unreachable!();
    }
}

fn main() {
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();

    let file_path = String::from("src/bin/day8.txt");
    let file = fs::read_to_string(file_path).unwrap();
    let contents: Vec<&str> = file.lines().collect();
    let directions = contents[0];
    //println!("{:?}", file);
    //println!("{:?}", directions);
    //println!("{:?}", contents);

    for i in 2..contents.len() {
        let re = Regex::new(r"[A-Z]+");
        let matches: Vec<&str> = re.unwrap().find_iter(&contents[i]).map(|c| c.as_str()).collect();
        //println!("{:?}", matches);
        nodes.insert(matches[0], [matches[1], matches[2]]);
    }
    //println!("{:?}", nodes);

    let result = "AAA";
    let steps: i32 = 0;
    let z = find_steps(&nodes, directions, result, steps);
    println!("It took this many steps: {:?}", z);
    
}
