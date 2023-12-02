use std::fs;

use std::cmp;



fn main() {
    let contents = fs::read_to_string("./03.input")
        .expect("Should have been able to read the file");

    let lines : Vec<&str> = contents.lines().collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut adjacent_map : Vec<Vec<bool>> = vec![vec![false; width]; height];

    let get_char = |h:usize, w:usize| -> char {
        lines[h].get(w..w+1).expect("Out of bounds").chars().next().expect("char???")
    };

    // Part 1
    // make adjacent map:
    for h in 0..height {
        for w in 0..width {
            let char = get_char(h, w);
            if ! (char == '.' || char.is_numeric()) {
                for i in (h-1)..(h+2) {
                    for j in (w-1)..(w+2) {
                        if i >= 0 && i < width && j >= 0 && j < height {
                            adjacent_map[i][j] = true;
                        }
                    }
                }
            }        
        }
    }

    let mut numbers : Vec<i32>= vec![];

    for h in 0..height {
        let mut w = 0;
        while w < width {
            let char = get_char(h,w);
            if char.is_numeric() {
                let mut we = w+1;
                while we < width && get_char(h, we).is_numeric() {
                    we = we +1;
                }
                let number : i32 = lines[h].get(w..we).expect("Out of bounds").parse().expect("Not a number");
                let adj = adjacent_map[h].get(w..we).unwrap().iter().any(|f| *f);
                //print!("{}, {:?}", number, adj);
                if adj {
                    numbers.push(number);
                }
                w = we - 1;
            }
            w = w+1;
        }
    }

    let sum : i32 = numbers.iter().sum();

    println!("{}",sum);

    //Part 2
    let mut res : Vec<i32> = vec![];
    let mut visited : Vec<Vec<bool>> = vec![vec![false; width]; height];
    for h in 0..height {
        for w in 0..width {
            if get_char(h, w) == '*' {
                let mut pairs : Vec<i32> = vec![];
                for i in (h-1)..(h+2) {
                    for j in (w-1)..(w+2) {
                        if i < width && j < height {
                            if get_char(i,j).is_numeric() && !visited[i][j] {
                                let (mut w1, mut w2) = (j,j);
                                while w1 > 0 && get_char(i,w1-1).is_numeric() {
                                    w1 = w1-1
                                }
                                while w2 < width-1 && get_char(i,w2+1).is_numeric() {
                                    w2 = w2+1
                                }
                                w2 = w2+1;
                                visited[i][w1..w2].fill(true);
                                pairs.push(lines[i].get(w1..w2).unwrap().parse().expect("numberr!!!"));
                            }
                        }
                    }
                }
                if pairs.len() == 2 {
                    res.push(pairs[0] * pairs[1]);
                }
            }        
        }
    }

    let sum : i32 = res.iter().sum();
    println!("{}",sum);
}

