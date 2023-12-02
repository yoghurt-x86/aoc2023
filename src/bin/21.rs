use std::{fs, collections::{VecDeque, HashSet, HashMap}};
use itertools::Itertools;

use std::cmp;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Dir {
    North,
    West,
    South,
    East,
}

fn main() {
    let contents = fs::read_to_string("./21.input2")
        .expect("Should have been able to read the file");

    let mut map : Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();


    let mut s = (0,0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                s = (x,y);
            }
            print!("{}",map[y][x]);
        }
        print!("{}",'\n');
    }



    print!("{:?}",s);


    let step = | map : &Vec<Vec<char>>, x, y | {
        let north =
            if y > 0 { (x, y-1) } else { (x, map.len()-1) };
        let south = 
            if y < (map.len() - 1) { (x,y+1) } else { (x, 0) };
        let west  = 
            if x > 0 { (x-1,y) } else { (map[0].len() - 1 ,y) };
        let east  = 
            if x < map[0].len() - 1 { (x+1,y) } else { (0, y) };

        let directions = vec![north,south,west,east].into_iter()
            .filter(|(dx,dy) : &(usize, usize)| map[*dy][*dx] != '#')
            .collect::<Vec<_>>();
        directions
    };


    let mut steps = HashMap::new();
    steps.insert(s, 1);

    for _ in 0..50 {
        let current_steps = steps;
        steps = HashMap::new();

        for (s, count) in current_steps {
            let current = step(&map, s.0, s.1);
            for c in current {
                steps.insert(c);
            }
        }
    }

    for s in &steps {
        map[s.1][s.0] = 'O';
    }

    print!("{}",'\n');
    step(&mut map, s.0, s.1);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}",map[y][x]);
        }
        print!("{}",'\n');
    }

    println!("{}", steps.len());
}

