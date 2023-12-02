use std::fs;
use itertools::Itertools;

use std::cmp;

fn main() {
    let contents = fs::read_to_string("./14.input")
        .expect("Should have been able to read the file");

    let map : Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let mut map = map.clone();

    let mut list = Vec::new();
    list.push(map.clone());
    for idx in 0..1000000000 {
        //north
        let mut movement = true;
        while movement {
            movement = false;
            for y in 1..map.len() {
                for x in 0..map[0].len() {
                    match (map[y][x], map[y-1][x]) {
                        ('O', '.') => {
                            map[y][x] = '.';
                            map[y-1][x] = 'O';
                            movement = true;
                            },
                        _ => {},
                    }
                }
            }
        }
        movement = true;
        // west
        while movement {
            movement = false;
            for x in 1..map[0].len() {
                for y in 0..map.len() {
                    match (map[y][x], map[y][x-1]) {
                        ('O', '.') => {
                            map[y][x] = '.';
                            map[y][x-1] = 'O';
                            movement = true;
                            },
                        _ => {},
                    }
                }
            }
        }
        // south
        movement = true;
        while movement {
            movement = false;
            for y in 0..(map.len()-1) {
                let y = (map.len()-2)-y;
                for x in 0..map[0].len() {
                    match (map[y][x], map[y+1][x]) {
                        ('O', '.') => {
                            map[y][x] = '.';
                            map[y+1][x] = 'O';
                            movement = true;
                            },
                        _ => {},
                    }
                }
            }
        }
        // east
        movement = true;
        while movement {
            movement = false;
            for x in 0..(map[0].len()-1) {
                let x = (map[0].len()-2)-x;
                for y in 0..map.len() {
                    match (map[y][x], map[y][x+1]) {
                        ('O', '.') => {
                            map[y][x] = '.';
                            map[y][x+1] = 'O';
                            movement = true;
                            },
                        _ => {},
                    }
                }
            }
        }

        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                match map[y][x] {
                    'O' => sum = sum + (map.len() - y),
                    _ => (),
                }
            }
        }

        //for line in &map {
        //    for c in line {
        //        print!("{}", c);
        //    }
        //    print!("{}", '\n');
        //}
        //print!("{}", '\n');

        //if previous_map == map {
        println!("cycle: {}, sum: {}", idx + 1, sum);
        let mut br = false;
        for (i,m) in list.iter().enumerate() {
            if *m == map {
                br = true;
                println!("map after {} cycles: match cycle! {}", i, idx + 1);
            }
        }
        if br {
            break;
        }
        list.push(map.clone());
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                'O' => sum = sum + (map.len() - y),
                _ => (),
            }
        }
    }
    println!("{}", sum);
}

