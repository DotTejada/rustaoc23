use std::{fs, iter::zip};

fn main() {
    let file_path: String = String::from("src/bin/day6.txt");
    let mut file: Vec<String> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().split("\n") {
        file.push(line.to_string())
    }

    // println!("{:#?}", file);

    let times: Vec<i32> = file[0][(file[0].find(':').unwrap() + 6)..]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // println!("{:?}", times);

    let distances: Vec<i32> = file[1][(file[1].find(':').unwrap() + 2)..]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // println!("{:?}", distances);

    let zipped = zip(times, distances);

    let mut ans: i32 = 1;
    for (time, dist) in zipped {
        // println!("({}, {})", time, dist);
        let mut ways: i32 = 0;

        // this is an optimization that makes it only have to compute half the numbers it normally
        // would, taking advantage of the fact that after the halfway mark the results are mirrored
        // 
        // i also figured out that when time is even, then if we find the number of ways before the
        // halfway point, we can just stop there, multiply by 2 for the mirror and add 1 for the
        // middle value we get
        //
        // when time is odd, we can just multiply by 2, it mirrors perfectly
        if time % 2 == 0 {
            for i in 0..=((time - 2) / 2) {
                let res: i32 = (time - i) * (i);
                // println!("{}: {}", i, res);
                if res > dist {
                    ways += 1
                }
            }
            ans *= (ways * 2) + 1;
            // println!("{} ways before the half.", ways)
        } else {
            for i in 0..=((time - 1) / 2) {
                let res: i32 = (time - i) * (i);
                // println!("{}: {}", i, res);
                if res > dist {
                    ways += 1
                }
            }
            ans *= ways * 2;
            // println!("{} ways before the half.", ways)
        }
    }
    println!("The answer is {}.", ans)
}
