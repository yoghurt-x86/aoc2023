use std::{fs, collections::{VecDeque, HashSet, BinaryHeap, HashMap}};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone)]
pub enum Dir {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Pos(usize, usize, Dir, usize);

fn main() {
    let contents = fs::read_to_string("./17.input")
        .expect("Should have been able to read the file");

    let map : Vec<Vec<usize>> = contents.lines()
        .map(|l| l.chars()
            .map(|i| usize::try_from(i.to_digit(10).unwrap()).unwrap()).collect()
        ).collect();

    for line in &map {
        for tile in line {
            print!("{}",tile);
        }
        print!("{}",'\n');
    }

    let directions2 = | (x,y,dir) : (usize, usize, Dir)| {
        let north  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(0,-4, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4)]), 
                 (0,-5, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5)]), 
                 (0,-6, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5), (0,-6)]),
                 (0,-7, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5), (0,-6), (0, -7)]),
                 (0,-8, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5), (0,-6), (0, -7), (0, -8)]),
                 (0,-9, Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5), (0,-6), (0, -7), (0, -8), (0, -9)]),
                 (0,-10,Dir::North, vec![(0,-1),(0,-2),(0,-3),(0, -4), (0, -5), (0,-6), (0, -7), (0, -8), (0, -9), (0, -10)]),
                ];
        let south  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(0,4, Dir::South, vec![(0,1),(0,2),(0,3),(0,4)]), 
                 (0,5, Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5)]), 
                 (0,6, Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5), (0,6)]),
                 (0,7, Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5), (0,6), (0, 7)]),
                 (0,8, Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5), (0,6), (0, 7), (0, 8)]),
                 (0,9, Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5), (0,6), (0, 7), (0, 8), (0, 9)]),
                 (0,10,Dir::South, vec![(0,1),(0,2),(0,3),(0,4), (0, 5), (0,6), (0, 7), (0, 8), (0, 9), (0, 10)]),
                ];
        let west  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(-4, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0)]), 
                 (-5, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0)]), 
                 (-6, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0), (-6,0)]),
                 (-7, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0), (-6,0), (-7,0)]),
                 (-8, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0), (-6,0), (-7,0), (-8,0)]),
                 (-9, 0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0), (-6,0), (-7,0), (-8,0), (-9,0)]),
                 (-10,0, Dir::West, vec![(-1,0),(-2,0),(-3,0),(-4,0), (-5,0), (-6,0), (-7,0), (-8,0), (-9,0), (-10,0)]),
                ];
        let east  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(4, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0)]), 
                 (5, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0)]), 
                 (6, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0), (6,0)]),
                 (7, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0), (6,0), (7,0)]),
                 (8, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0), (6,0), (7,0), (8,0)]),
                 (9, 0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0), (6,0), (7,0), (8,0), (9,0)]),
                 (10,0, Dir::East, vec![(1,0),(2,0),(3,0),(4,0), (5,0), (6,0), (7,0), (8,0), (9,0), (10,0)]),
                ];

        let directions = match dir {
            Dir::North => vec![west, east].concat(),
            Dir::South => vec![west, east].concat(),
            Dir::West => vec![north, south].concat(),
            Dir::East => vec![north, south].concat(),
        }.into_iter()
            .filter_map(|cock| {
                let inside = |x,y| {
                    let _x = cock.0 + i32::try_from(x).unwrap();
                    let _x = 
                        if _x < 0 || _x >= (map[0].len().try_into().unwrap()) {
                            None
                        } else {
                            Some(_x)
                        };
                    let _y = cock.1 + i32::try_from(y).unwrap();
                    let _y = 
                        if _y < 0 || _y >= (map.len().try_into().unwrap()) {
                            None
                        } else {
                            Some(_y)
                        };
                    _x.zip(_y)
                };

                inside(x,y).map(| (_x, _y)| {
                    let cost = cock.3.into_iter().fold(0, | c, (cx, cy)| {
                            c + map[usize::try_from(cy + i32::try_from(y).unwrap()).unwrap()][usize::try_from(cx + i32::try_from(x).unwrap()).unwrap()]
                        }
                    );
                    (usize::try_from(_x).unwrap(), usize::try_from(_y).unwrap(), cock.2, cost)
                })
            }).collect::<Vec<_>>();
        directions
    };


    let directions = | (x,y,dir) : (usize, usize, Dir)| {
        let north  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(0,-1,Dir::North, vec![(0,-1)]), 
                 (0,-2,Dir::North, vec![(0, -1), (0, -2)]), 
                 (0,-3,Dir::North, vec![(0, -1), (0, -2), (0,-3)])
                ];
        let south  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(0,1,Dir::South, vec![(0,1)]), 
                 (0,2,Dir::South, vec![(0,1), (0,2)]), 
                 (0,3,Dir::South, vec![(0,1), (0,2), (0,3)])
                ];
        let west  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(-1,0,Dir::West, vec![(-1,0)]), 
                 (-2,0,Dir::West, vec![(-1,0), (-2,0)]), 
                 (-3,0,Dir::West, vec![(-1,0), (-2,0), (-3,0)])
                ];
        let east  :  Vec<(i32, i32, Dir, Vec<(i32,i32)>)>= 
            vec![(1,0,Dir::East, vec![(1,0)]), 
                 (2,0,Dir::East, vec![(1,0), (2,0)]), 
                 (3,0,Dir::East, vec![(1,0), (2,0), (3,0)])
                ];

        let directions = match dir {
            Dir::North => vec![west, east].concat(),
            Dir::South => vec![west, east].concat(),
            Dir::West => vec![north, south].concat(),
            Dir::East => vec![north, south].concat(),
        }.into_iter()
            .filter_map(|cock| {
                let inside = |x,y| {
                    let _x = cock.0 + i32::try_from(x).unwrap();
                    let _x = 
                        if _x < 0 || _x >= (map[0].len().try_into().unwrap()) {
                            None
                        } else {
                            Some(_x)
                        };
                    let _y = cock.1 + i32::try_from(y).unwrap();
                    let _y = 
                        if _y < 0 || _y >= (map.len().try_into().unwrap()) {
                            None
                        } else {
                            Some(_y)
                        };
                    _x.zip(_y)
                };

                inside(x,y).map(| (_x, _y)| {
                    let cost = cock.3.into_iter().fold(0, | c, (cx, cy)| {
                            c + map[usize::try_from(cy + i32::try_from(y).unwrap()).unwrap()][usize::try_from(cx + i32::try_from(x).unwrap()).unwrap()]
                        }
                    );
                    (usize::try_from(_x).unwrap(), usize::try_from(_y).unwrap(), cock.2, cost)
                })
            }).collect::<Vec<_>>();
        directions
    };

    //let mut dist : HashMap<(usize,usize,Dir), usize>= HashMap::new();

    impl Ord for Pos {
        fn cmp(&self, other: &Self) -> Ordering {
            // Notice that the we flip the ordering on costs.
            // In case of a tie we compare positions - this step is necessary
            // to make implementations of `PartialEq` and `Ord` consistent.
            //other.2.cmp(&self.2)
            other.3.cmp(&self.3)
                .then_with(|| self.2.cmp(&other.2))
        }
    }
    impl PartialOrd for Pos {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut dist : HashMap<(usize,usize, Dir), usize> = HashMap::new();
    let mut prev : HashMap<(usize,usize, Dir), (usize, usize, Dir)> = HashMap::new();

    let mut heap : BinaryHeap<Pos>= BinaryHeap::new();

    heap.push(Pos(0,0,Dir::South, 0));
    heap.push(Pos(0,0,Dir::East, 0));
    dist.insert((0,0,Dir::South), 0);
    dist.insert((0,0,Dir::East), 0);

    while let Some(Pos(x,y,dir,cost)) = heap.pop() {
        if x == (map[0].len() - 1) && (y == map.len() - 1 ) {
            break;
        }

        if let Some(d)= dist.get(&(x,y,dir.clone())) {
            if *d < cost {
                continue;
            }
        }

        let nexts = directions2((x,y,dir.clone()));

        //println!("({},{},{:?}) {:?}", x,y,dir, nexts);
        //println!("{:?}", heap);
        //println!("{:?}", dist);

        for (nx,ny,ndir,c) in nexts {
            let d = dist.get(&(nx,ny,ndir.clone())).unwrap_or(&usize::MAX);
            let ncost = c + cost;

            if *d > ncost {
                heap.push(Pos(nx,ny,ndir.clone(),ncost));
                dist.insert((nx,ny,ndir.clone()),ncost);
                prev.insert((nx, ny,ndir), (x,y, dir.clone()));
            }
        }
    }



    let mut visited_map : Vec<Vec<Option<Dir>>>= vec![vec![None;map[0].len()];map.len()];

    let mut x = map[0].len()-1;
    let mut y = map.len()-1;
    let mut dir = Dir::East;
    while let Some(g) = prev.get(&(x,y,dir)) {
        x = g.0;
        y = g.1;
        dir = g.2.clone();
        visited_map[y][x] = Some(g.2.clone());
    }

    println!("east {:?}", dist.get(&(map[0].len()-1, map.len()-1, Dir::East)));
    println!("south {:?}", dist.get(&(map[0].len()-1, map.len()-1, Dir::South)));
    println!("{:?}", prev.get(&(map[0].len()-1, map.len()-1, Dir::East)));

    for y in 0..map.len() {
        for x in 0..map.len() {
            match visited_map[y][x] {
                Some(Dir::North) => print!("{}", '^'),
                Some(Dir::West) => print!("{}", '<'),
                Some(Dir::South) => print!("{}", 'v'),
                Some(Dir::East) => print!("{}", '>'),
                None => print!("{}",map[y][x]),
            }
        }
        print!("{}",'\n');
    }
}

