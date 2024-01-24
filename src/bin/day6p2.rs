use std::fs;

fn main() {
    let file_path: String = String::from("src/bin/day6.txt");
    let mut file: Vec<String> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().split("\r\n") {
        file.push(line.to_string())
    }

    println!("{:#?}", file);

    let time: u64 = file[0][(file[0].find(':').unwrap() + 6)..]
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    println!("{:?}", time);

    let distance: u64 = file[1][(file[1].find(':').unwrap() + 2)..]
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    println!("{:?}", distance);

    let mut ans: u64 = 1;
        // println!("({}, {})", time, dist);
        let mut ways: u64 = 0;

        if time % 2 == 0 {
            for i in 0..=((time - 2) / 2) {
                let res: u64 = (time - i) * (i);
                // println!("{}: {}", i, res);
                if res > distance {
                    ways += 1
                }
            }
            ans *= (ways * 2) + 1;
            // println!("{} ways before the half.", ways)
        } else {
            for i in 0..=((time - 1) / 2) {
                let res: u64 = (time - i) * (i);
                // println!("{}: {}", i, res);
                if res > distance {
                    ways += 1
                }
            }
            ans *= ways * 2;
            // println!("{} ways before the half.", ways)
        }
    println!("The answer is {}.", ans)
}
