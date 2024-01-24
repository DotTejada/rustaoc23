use std::{fs, i64::MAX};

fn main() {
    let file_path: String = String::from("src/bin/day5.txt");

    let mut file: Vec<String> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().split("\r\n\r\n") {
        file.push(line.to_string())
    }

    // println!("{:#?}", file);

    let mut seeds: Vec<i64> = file[0][(file[0].find(':').unwrap() + 2)..]
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // println!("{:?}", seeds);

    let maps: Vec<Vec<Vec<i64>>> = file[1..8]
        .iter()
            .map(|line| line.split("\r\n").skip(1)
                .map(|elem| elem.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect())
                .collect())
            .collect();
    
    // println!("{:?}", maps);

    let mut lowest: i64 = MAX;
    for seed in &mut seeds{
        for map in &maps {
            for line in map {
                let dest = line[0]..(line[0] + line[2]);
                let src = line[1]..(line[1] + line[2]);
                if src.contains(&seed) {
                    *seed += (dest.min().unwrap()) - (src.min().unwrap());
                    break;
                }
            }
        }
        if seed < &mut lowest {
               lowest = *seed; 
            }
    }

    println!("{:?}", lowest);
}
