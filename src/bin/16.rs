use std::{fs, collections::{VecDeque, HashSet}};
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
    let contents = fs::read_to_string("./16.input")
        .expect("Should have been able to read the file");

    let map : Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    for line in &map {
        for tile in line {
            print!("{}",tile);
        }
        print!("{}",'\n');
    }

    let goto = | x : usize, y : usize, dir | -> Vec<(usize,usize,Dir)>{
        let north : (i32, i32)= (0,-1);
        let south : (i32, i32)= (0,1);
        let west  : (i32, i32)= (-1,0);
        let east  : (i32, i32)= (1,0);
        let char : char = map[y][x];

        let dirs = match (char, dir) {
            ('/', Dir::North) => vec![Dir::East],
            ('/', Dir::South) => vec![Dir::West],
            ('/', Dir::East) => vec![Dir::North],
            ('/', Dir::West) => vec![Dir::South],
            ('\\', Dir::North) => vec![Dir::West],
            ('\\', Dir::South) => vec![Dir::East],
            ('\\', Dir::East) => vec![Dir::South],
            ('\\', Dir::West) => vec![Dir::North],
            ('.', Dir::North) => vec![Dir::North],
            ('.', Dir::South) => vec![Dir::South],
            ('.', Dir::East) => vec![Dir::East],
            ('.', Dir::West) => vec![Dir::West],
            ('|', d) => match d {
                Dir::North => vec![Dir::North],
                Dir::South => vec![Dir::South],
                Dir::West | Dir::East => vec![Dir::North, Dir::South],
            },
            ('-', d) => match d {
                Dir::West => vec![Dir::West],
                Dir::East => vec![Dir::East],
                Dir::North | Dir::South => vec![Dir::East, Dir::West],
            },
            (s,_) => panic!("Unknown char in goto: {:?}", s),
        };

        let res = dirs.iter()
            .map(|penis| {
                match penis {
                    Dir::North => (north, Dir::North),
                    Dir::West => (west, Dir::West),
                    Dir::South => (south, Dir::South),
                    Dir::East => (east, Dir::East),
                }
            }).filter(|balls| {
                match balls.1 {
                    Dir::North => y != 0,
                    Dir::West => x != 0,
                    Dir::South => y < map.len() - 1,
                    Dir::East => x < map[0].len() -1,
                }
            }).map(|cock| {
                let _x = cock.0.0 + i32::try_from(x).unwrap();
                let _y = cock.0.1 + i32::try_from(y).unwrap();
                (usize::try_from(_x).unwrap(), usize::try_from(_y).unwrap(), cock.1)
            }).collect::<Vec<_>>();

        res
    };



    let dick = | start : (usize,usize,Dir) | {
        let mut lights_todo = VecDeque::from(vec![start]);
        let mut visited_map = vec![vec![false;map[0].len()];map.len()];
        let mut visited = HashSet::new();

        while let Some(l) = lights_todo.pop_front() {
            visited.insert(l.clone());
            visited_map[l.1][l.0] = true;

            let possible_directions = goto(l.0, l.1, l.2);
            for dir in possible_directions{
                if !visited.contains(&dir) {
                    lights_todo.push_back(dir);
                }

            }

        }

        //for y in 0..map.len() {
        //    for x in 0..map[0].len() {
        //        if visited_map[y][x] {
        //            print!("{}", '#');
        //        } else {
        //            print!("{}",map[y][x]);
        //        }
        //    }
        //    print!("{}",'\n');
        //}

        let mut sum = 0;

        for line in visited_map {
            for t in line {
                if t {
                    sum = sum + 1;
                }
            }
        }
        sum
    };

    let xs = (0..map[0].len()).map(|x| vec![(x,0, Dir::South), (x, map.len() - 1, Dir::North)]).flatten();
    let ys = (0..map.len()).map(|y| vec![(0,y, Dir::East), (map[0].len() - 1, y, Dir::West)]).flatten();
    let startings = xs.chain(ys);

    let sums = startings.map(|s| dick(s));

    let sum = sums.max();


    println!("SUm {:?}", sum);
}

