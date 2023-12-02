use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;

use std::cmp;


extern crate combine;
use combine::sep_end_by1;
use combine::{between, many1, Parser, sep_by, choice, many};
use combine::parser::range::{take_while};
use combine::parser::char::{newline, spaces, char, digit, space};

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Dir {
    North,
    West,
    South,
    East,
}

fn main() {
    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let hex = || char('#').with(take_while(|c: char| c.is_ascii_hexdigit()));
    let dir = || choice((
        char('U'), 
        char('D'), 
        char('R'), 
        char('L')
    )).map(|c| match c {
            'U' => Dir::North,
            'D' => Dir::South,
            'R' => Dir::East,
            'L' => Dir::West,
            _ => panic!("unexpected char"),
        });

    let line = || dir()
        .skip(space())
        .and(integer())
        .skip(space())
        .and(between(char('('), char(')'), hex()))
        .skip(newline());

    let content = || many1(line())
        .map(| l : Vec<_>| l);

    let contents = fs::read_to_string("./18.input.backup")
        .expect("Should have been able to read the file");

    let blah : Vec<_>= content().parse(contents.as_str()).unwrap().0;

    let mut points = Vec::new();

    let mut x = 0;
    let mut y = 0;
    points.push((x,y));
    for ((dir, l),_) in &blah {
        for step in 0..*l {
            let (_x, _y) = match dir {
                Dir::North => (x,y - 1),
                Dir::South => (x,y + 1),
                Dir::West => (x - 1, y),
                Dir::East => (x + 1, y),
            };
            x = _x;
            y = _y;
            
            points.push((x,y))
        }
    }


    let (minx, maxx) = points.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (miny, maxy) = points.iter().map(|p| p.1).minmax().into_option().unwrap();


    let mut upoints : Vec<(usize, usize)> = points.iter().map(|(x,y)| (usize::try_from(x - minx).unwrap(), usize::try_from(y - miny).unwrap())).collect();

    let mut map = vec![ vec![ false; (maxx - minx + 1).try_into().unwrap()];(maxy - miny + 1).try_into().unwrap()];

    upoints.iter().for_each(|(x,y)| map[*y][*x] = true);

    println!("{}", &contents);
    println!("{:?}", blah);
    println!("{:?}", points);

    let around = | map: &Vec<Vec<_>>, (cx, cy) : (usize, usize) | {
        let south = if cy >= (map.len() - 1) {None} else {Some((cx,cy + 1))};
        let north = if cy == 0 {None} else {Some((cx,cy - 1))};
        let west = if cx == 0 { None } else { Some((cx-1,cy))};
        let east = if cx >= (map[0].len() - 1) { None } else {Some((cx+1,cy))};
        vec![north, south, east, west].into_iter().flatten().collect::<Vec<_>>()
    };

    let mut queue : VecDeque<(usize,usize)>= VecDeque::new();
    queue.push_back((map[0].len() / 2, map.len() / 2));

    while let Some((x,y)) = queue.pop_back() {
        if map[y][x] {
            continue;
        } else {
            map[y][x] = true;
            let arounds = around(&map, (x,y));
            for a in arounds {
                queue.push_back(a);
            }
        }
    }


    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                sum = sum + 1;
                print!("{}", '#');
            } else {
                print!("{}", '.');
            }
        }
        print!("{}", '\n');
    }
    println!("{:?}", sum);

    let mut points2 : Vec<(i32,i32,Dir)> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for ((dir, l),_) in &blah {
        points2.push((x,y,dir.clone()));
        let (_x, _y) = match dir {
            Dir::North => (x,y - l),
            Dir::South => (x,y + l),
            Dir::West => (x - l, y),
            Dir::East => (x + l, y),
        };
        x = _x;
        y = _y;
    }
    points2.push(points2[0]);
    
    let (mut l, r) : (Vec<_>, Vec<_>) = points2.iter().tuple_windows().map(|((x1,y1,d1), (x2,y2,d2))| {
        let southwest = (-0.5,0.5);
        let southeast = (0.5,0.5);
        let northeast = (0.5,-0.5);
        let northwest = (-0.5,-0.5);

        let (l,r) = match d1 {
            Dir::North => match d2 {
                Dir::West => (southwest, northeast),
                Dir::East => (northwest, southeast),
                _ => panic!("fuck north"),
            },
            Dir::East  => match d2 {
                Dir::North => (northwest, southeast),
                Dir::South => (northeast, southwest),
                _ => panic!("fuck east"),
            },
            Dir::South  => match d2 {
                Dir::West => (southeast, northwest),
                Dir::East => (northeast, southwest),
                _ => panic!("fuck south"),
            },
            Dir::West  => match d2 {
                Dir::North => (southwest, northeast),
                Dir::South => (southeast, northwest),
                _ => panic!("fuck west"),
            },
        };
        ((f64::from(*x2) + l.0, f64::from(*y2) + l.1),(f64::from(*x2) + r.0, f64::from(*y2) + r.1))
    }).unzip();


    //let area : i32 = points2.iter().tuple_windows().map(|((x1,y1,_), (x2,y2,_))|
    //        (i32::try_from(*x1).unwrap() * i32::try_from(*y2).unwrap()) - (i32::try_from(*y1).unwrap() * i32::try_from(*x2).unwrap())
    //    ).sum();

    l.push(l[0]);
    let area2 : f64 = l.iter().tuple_windows().map(|((x1,y1), (x2,y2,))|
            (x1 * y2) - (y1 * x2)
        ).sum();

    println!("{:?}", area2 / 2.0 );
}

