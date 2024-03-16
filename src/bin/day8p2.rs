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

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_multiple(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0; // LCM is undefined for an empty list of numbers
    }

    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}

fn main() {
    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();

    let file_path = String::from("C:/Users/dotdu/rustaoc23/src/bin/day8.txt");
    let file = fs::read_to_string(file_path).unwrap();
    let contents: Vec<&str> = file.lines().collect();
    let directions = contents[0];
    //println!("{:?}", file);
    //println!("{:?}", directions);
    //println!("{:?}", contents);

    for i in 2..contents.len() {
        let re = Regex::new(r"[^\s()=,]+");
        let matches: Vec<&str> = re.unwrap().find_iter(&contents[i]).map(|c| c.as_str()).collect();
        //println!("{:?}", matches);
        nodes.insert(matches[0], [matches[1], matches[2]]);
    }
    //println!("{:?}", nodes);
    let mut a = Vec::new();
    for (i, [_, _]) in &nodes {
        if i.chars().nth_back(0).unwrap() == 'A' {
            a.push(*i);
        }
    }
    println!("{:?}", a);

    let mut steps: u64 = 0;
    let mut z = Vec::new();
    for mut node in a {
        loop {
            for d in directions.chars() {
                if node.chars().nth_back(0).unwrap() == 'Z' {
                    z.push(steps);
                    steps = 0;
                    break
                }
                let x = find_next(&nodes, node, d);
                node = x;
                steps += 1;
            }
            if node.chars().nth_back(0).unwrap() == 'Z' {
                z.push(steps);
                steps = 0;
                break
            }
        }
    }

    println!("It took this many steps: {:?}", z);
    let res = lcm_multiple(z);
    println!("It took this many steps: {:?}", res);
    
}
