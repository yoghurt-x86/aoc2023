use std::fs;
use itertools::Itertools;
use std::collections::VecDeque;



fn find(map: &Vec<Vec<char>>, c : char) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, l)| l.iter().enumerate().find_map(|(x, m)| if *m == c { Some((x,y))} else {None} ))
            }

fn connected_paths(map: &Vec<Vec<char>>, (x,y) : (usize, usize)) -> Vec<(usize,usize)> {
    let check = |(cx,cy) : (usize,usize)| { 
        let south = if cy >= (map.len() - 1) {None} else {Some((cx,cy + 1))};
        let north = if cy == 0 {None} else {Some((cx,cy - 1))};
        let west = if cx == 0 { None } else { Some((cx-1,cy))};
        let east = if cx >= (map[0].len() - 1) { None } else {Some((cx+1,cy))};

        //println!("{:?}: n{:?} s{:?} e{:?} w{:?}", (cx,cy), north, south, east, west);

        match map[cy][cx] {
            'S' => vec![north, south, east, west],
            '-' => vec![east, west],
            '|' => vec![north, south],
            'F' => vec![east, south],
            'J' => vec![north, west],
            '7' => vec![west, south],
            'L' => vec![east, north],
            _ => vec![]
        }.into_iter().flatten().collect::<Vec<_>>()
    };

    let checks = check((x,y));

    let paths = checks.into_iter().filter(|(cx,cy)| {
        let bah = check((*cx, *cy));
        //println!("{:?}", bah);
        bah.iter().any(|(bx,by)| *bx == x && *by == y) 
    });

    paths.collect::<Vec<_>>()
}

fn follow_pipe(map: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, (x,y) : (usize, usize)) -> Option<(usize,usize)> {
    let check = |(cx,cy) : (usize,usize)| { 
        let south = if cy >= (map.len() - 1) {None} else {Some((cx,cy + 1))};
        let north = if cy == 0 {None} else {Some((cx,cy - 1))};
        let west = if cx == 0 { None } else { Some((cx-1,cy))};
        let east = if cx >= (map[0].len() - 1) { None } else {Some((cx+1,cy))};

        match map[cy][cx] {
            'S' => vec![north, south, east, west],
            '-' => vec![east, west],
            '|' => vec![north, south],
            'F' => vec![east, south],
            'J' => vec![north, west],
            '7' => vec![west, south],
            'L' => vec![east, north],
            _ => vec![]
        }.into_iter().flatten().filter(|p| !visited[p.1][p.0]).nth(0)
    };
    check((x,y))
}

pub enum Dir {
    West,
    East, 
    North,
    South,
}

fn marks(map: &Vec<Vec<char>>, (x,y) : (usize, usize), before: (usize, usize)) -> (Vec<(usize,usize)>,Vec<(usize,usize)>) {
    let signed = |i:usize| i32::try_from(i).unwrap();
    let diff = (signed(before.0) - signed(x), signed(before.1) - signed(y));

    let south = if y >= (map.len() - 1) {None} else {Some((x,y + 1))};
    let north = if y == 0 {None} else {Some((x,y - 1))};
    let west = if x == 0 { None } else { Some((x-1,y))};
    let east = if x >= (map[0].len() - 1) { None } else {Some((x+1,y))};

    let from = match diff {
        (1,0) => Dir::East,
        (-1,0) => Dir::West,
        (0,1) => Dir::South,
        (0,-1) => Dir::North,
        _ => panic!("Impossible!!!"),
    };

    let (left, right) : (Vec<_>, Vec<_>) = { 
        let asd = match map[y][x] {
            'S' => (vec![], vec![]),
            '|' => match from {
                    Dir::North => (vec![east], vec![west]),
                    Dir::South => (vec![west], vec![east]),
                    _ => panic!("| Direction!!!"),
                }
            '-' => match from {
                    Dir::West => (vec![north], vec![south]),
                    Dir::East => (vec![south], vec![north]),
                    _ => panic!("- Direction!!!"),
                }
            'F' => match from {
                    Dir::East => (vec![], vec![north, west]),
                    Dir::South => (vec![north, west], vec![]),
                    _ => panic!("F Direction!!!"),
                }
            'J' => match from {
                    Dir::West => (vec![], vec![east, south]),
                    Dir::North => (vec![east, south], vec![]),
                    _ => panic!("J Direction!!!"),
                }
            '7' => match from {
                    Dir::South => (vec![], vec![east, north]),
                    Dir::West => (vec![east, north], vec![]),
                    _ => panic!("7 Direction!!!"),
                }
            'L' => match from {
                    Dir::North => (vec![], vec![west, south]),
                    Dir::East => (vec![west, south], vec![]),
                    _ => panic!("L Direction!!!"),
                }
            _ => panic!("unexpected!")
        };
        (asd.0.into_iter().flatten().collect(), asd.1.into_iter().flatten().collect())
    };
    (left,right)
}


fn main() {
    let contents = fs::read_to_string("./10.input")
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let map : Vec<Vec<char>> = lines.map(|l|l.chars().collect()).collect();
    let mut visited : Vec<Vec<bool>> = vec![vec![false;map[0].len()]; map.len()];
    let mut counts : Vec<Vec<i32>> = vec![vec![0;map[0].len()]; map.len()];

    let (sx, sy) = find(&map, 'S').unwrap();

    let paths = connected_paths(&map, (1,1));

    println!("part 1 {:?},{},{:?}", sx, sy, map[sy][sx]);
    println!("part 1 {:?},{},{:?}", paths, sy, map[sy][sx]);


    let mut count = 0;
    let mut places_to_go_next = VecDeque::from(vec![(sx,sy)]);

    while !places_to_go_next.is_empty() {
        let places = places_to_go_next.clone();
        places_to_go_next = VecDeque::new();

        for p in places {
            visited[p.1][p.0] = true;
            counts[p.1][p.0] = count;
            let connections = connected_paths(&map, p);
            for c in connections{
                if !visited[c.1][c.0] {
                    places_to_go_next.push_back(c);
                }
            }
        }
        count = count + 1;
    };

    println!("part1 {:?}", counts.into_iter().map(|l| l.into_iter().max().unwrap()).max());

    // part 2

    let around = | (cx, cy) : (usize, usize) | {
        let south = if cy >= (map.len() - 1) {None} else {Some((cx,cy + 1))};
        let north = if cy == 0 {None} else {Some((cx,cy - 1))};
        let west = if cx == 0 { None } else { Some((cx-1,cy))};
        let east = if cx >= (map[0].len() - 1) { None } else {Some((cx+1,cy))};
        vec![north, south, east, west].into_iter().flatten().collect::<Vec<_>>()
    };


    let mut marked : Vec<Vec<bool>> = vec![vec![false;map[0].len()]; map.len()];
    let mut places_to_go_next = VecDeque::from(vec![(0,0)]);

    while !places_to_go_next.is_empty() {
        let p = places_to_go_next.pop_back().unwrap();
        marked[p.1][p.0] = true;

        let arounds = around(p);
        for a in arounds {
            if (!visited[a.1][a.0]) && (!marked[a.1][a.0]) {
                places_to_go_next.push_back(a)
            }
        }
    }

    let mut count : i32 = 0;
    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            if visited[y][x] {
                print!(".");
            } else if marked[y][x] {
                print!("X");
            } else {
                count = count + 1;
                print!("O");
            }

        }
        println!("");
    }


    let mut visited : Vec<Vec<bool>> = vec![vec![false;map[0].len()]; map.len()];
    let mut marked : Vec<Vec<bool>> = vec![vec![false;map[0].len()]; map.len()];
    let mut mmap : Vec<Vec<char>> = vec![vec!['.';map[0].len()]; map.len()];//map.clone();
    visited[sy][sx] = true;
    let (mut x, mut y) = (sx, sy);
    while let Some(p) = follow_pipe(&map, &visited, (x, y)) {
        let (left, right) = marks(&map, p, (x,y));
        for l in left {
            mmap[l.1][l.0] = 'L'
        }
        for r in right {
            mmap[r.1][r.0] = 'R'
        }
        visited[p.1][p.0] = true;
        (x, y) = p;
    }


    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            if !marked[y][x] && !visited[y][x] && (mmap[y][x] == 'R' || mmap[y][x] == 'L'){
                let mut places_to_go_next = VecDeque::from(vec![(x,y)]);

                while !places_to_go_next.is_empty() {
                    let p = places_to_go_next.pop_back().unwrap();
                    marked[p.1][p.0] = true;
                    mmap[p.1][p.0] = mmap[y][x];

                    let arounds = around(p);
                    for a in arounds {
                        if (!visited[a.1][a.0]) && (!marked[a.1][a.0]) {
                            places_to_go_next.push_back(a)
                        }
                    }
                }
            }
        }
    }


    let mut count = 0;
    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            if visited[y][x] {
                //print!("{}", map[y][x]);
                print!(".");
            } else {
                print!("{}", mmap[y][x]);
            }
            if !visited[y][x] && mmap[y][x] == 'R' {
                count = count + 1;
            }

        }
        println!("");
    }
    println!("part2: {}", count);
}

