use std::fs;
use itertools::Itertools;

use std::cmp;


extern crate combine;
use combine::sep_end_by1;
use combine::{many1, Parser, sep_by, choice, many};
use combine::parser::char::{newline, spaces, char, digit};

fn main() {

    let row = || many1(char('#').or(char('.')));

    let table = || many1(row().skip(newline())).map(|rows : Vec<Vec<char>>| rows);

    let all = || sep_by(table(), newline());


    let contents = fs::read_to_string("./13.input")
        .expect("Should have been able to read the file");

    let blah : (Vec<Vec<Vec<char>>>,_)= all().parse(contents.as_str()).unwrap();

    println!("contents {:?}", blah);

    let mut sum = 0;
    let mut sum2 = 0;

    for board in blah.0 {
        let width = board[0].len();
        let height = board.len();
        let board_inv = {
            let mut inv = vec![Vec::new();board[0].len()];
            for w in 0..width {
                for h in 0..height {
                    inv[w].push(board[h][w]);
                }
            }
            inv
        };

        let find_folds = | board : &Vec<Vec<char>> | {
            board.into_iter()
                .enumerate()
                .tuple_windows().map(|((i1, _), (i2,_))| {
                let mut j1 = i1;
                let mut j2 = i2;
                let mut count = 0;
                while board[j1].iter().zip(board[j2].iter()).all(|(c1, c2)| c1 == c2) {
                    count = count + 1;
                    println!("{}, omg {} and {}", count, j1, j2);
                    if j1 == 0 || j2 >= (board.len()-1) {
                        return (count, true, i32::try_from(i1).unwrap());
                    }
                    j1 = j1 - 1;
                    j2 = j2 + 1;
                }
                if count != 0 {
                    println!("count: {}", count);
                }
                (count, false,i32::try_from(i1).unwrap())
            }).find(|(countacc, toedge, idxacc), | *toedge )
        };

        let find_folds_part2 = | board : &Vec<Vec<char>> | {
            let mut finds = Vec::new();

            let old = find_folds(board);

            for y in 0..board.len() {
                for x in 0..board[0].len() {
                    let mut board = board.clone();
                    board[y][x] = match board[y][x] {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!("Unknown character"),
                    };

                    let f = board.iter()
                        .enumerate()
                        .tuple_windows().map(|((i1, _), (i2,_))| {
                        let mut j1 = i1;
                        let mut j2 = i2;
                        let mut count = 0;
                        while board[j1].iter().zip(board[j2].iter()).all(|(c1, c2)| c1 == c2) {
                            count = count + 1;
                            //println!("{}, omg {} and {}", count, j1, j2);
                            if j1 == 0 || j2 >= (board.len()-1) {
                                match old {
                                    Some(o) => { 
                                        if o.2 == i32::try_from(i1).unwrap()
                                            { return None } 
                                        else 
                                            { return Some((count, true, i32::try_from(i1).unwrap()));}
                                        },

                                    None => { return Some((count, true, i32::try_from(i1).unwrap()));},
                                }
                            }
                            j1 = j1 - 1;
                            j2 = j2 + 1;
                        }
                        None
                    }).flatten().find(|(countacc, toedge, idxacc), | *toedge );
                    finds.push(f);
                }
            }
            let res = finds.into_iter().flatten().unique().collect::<Vec<_>>();
            if res.len() > 1 {
                println!("!!!! {:?}, old: {:?}", res, old);
                res.into_iter().filter(|r| Some(*r) != old).unique().nth(0)
            } else {
                res.into_iter().nth(0)
            }
        };

        let fh = find_folds(&board);
        let fv = find_folds(&board_inv);

        let f = match (fh, fv) {
            (Some(f), None) => (f.2 + 1) * 100,
            (None, Some(f)) => f.2 + 1,
            _ => panic!("asdasd"),
        };


        println!("f1: {:?} {:?} {:?}", f, fh, fv);

        sum = sum + f;
        
        let fh2 = find_folds_part2(&board);
        let fv2 = find_folds_part2(&board_inv);



        println!("fh2: {:?}", fh2);
        println!("fv2: {:?}", fv2);

        let f2 = match (fh2, fv2) {
            (Some(f), None) => (f.2 + 1) * 100,
            (None, Some(f)) => f.2 + 1,
            (Some(h), Some(v)) => {
                println!("pancid? {:?} {:?}", fh2, fv2);
                match (fh, fv) {
                    (Some(_), None) => v.2 + 1,
                    (None, Some(_)) => (h.2 + 1) * 100,
                    _ => panic!("weird!!!")
                }},
            _ => panic!("asdasdasd"),
        };

        println!("f2: {}", f2);

        if f == f2 {
            println!("nooo: {} {}", f, f2);
        }

        sum2 = sum2 + f2;

        println!("b: {:?}, ", f2);

        println!("board");
        for row in board {
            for c in row {
                print!("{}",c);
            }
            print!("{}",'\n');
        }
        print!("{}",'\n');
    }
    println!("part 1 {}",sum);
    println!("part 2 {}",sum2);
}

