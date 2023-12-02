use std::fs;

fn main() {
    let input : Vec<(i64,i64)>= vec![(48938466, 261119210191063)];

    let mut counts : Vec<u64> = Vec::new();
    for (time, distance) in input {
        let mut count : u64 = 0;
        for t in 0..time {
            let dist = t * (time - t);
            if dist > distance {
                count = count + 1;
            }
        }
        counts.push(count);
        println!("{:?}", count);
    };

    let res = counts.iter().fold(1, |a, b| a * b);
    println!("part 1 {:?}", res);
}

