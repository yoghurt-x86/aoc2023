use std::fs;

extern crate combine;
use combine::{choice, many1, Parser, sep_by};
use combine::parser::char::{spaces, string, char, digit};
use std::cmp;


#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq)]
pub struct Set {
    red : i32, 
    green : i32, 
    blue : i32, 
}

fn main() {
    let to_set = | list : Vec<(i32, Color)> | {
        let mut set = Set {red:0, green: 0, blue: 0};
        for (i,c) in list {
            match c {
                Color::Red => set.red = i,
                Color::Green => set.green = i,
                Color::Blue => set.blue = i,
            }
        };
        set
    };
    let lex_char = |c| char(c).skip(spaces());

    let color = || choice((
        string("red").map(|_| Color::Red), 
        string("green").map(|_| Color::Green), 
        string("blue").map(|_| Color::Blue)
    ));

    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let cubes = || spaces().with((integer(), spaces(), color()))
        .map(|(i, _ , c)| (i, c));
    let set = || spaces().with(sep_by(cubes(), lex_char(','))
        .map(|list : Vec<(i32, Color)>| to_set(list)));
    let sets = || sep_by(set(), lex_char(';'))
        .map(|list : Vec<Set>| list);
        
    let game = || string("Game").with(spaces()).with(integer());
    let line_parser = || (game(), lex_char(':'), sets()).map(|(id, _ , game)| (id, game));

    let contents = fs::read_to_string("./02.input")
        .expect("Should have been able to read the file");

    let lines1 = contents.lines().clone();
    let lines2 = contents.lines();

    let res : i32 = lines1.map( |l| {
        let (id, sets) = line_parser().parse(l).expect("line could not be parsed").0;
        let game_possible = 
                sets.iter().fold(true, 
                    | t, s | t && 
                        s.red <= 12 && 
                        s.green <= 13 && 
                        s.blue <= 14 
                    );
        if game_possible {
            id
        } else {
            0
        }

    }).sum();
    println!("part 1 {:?}", res);

    let res : i32 = lines2.map( |l| {
        let (_, sets) = line_parser().parse(l).expect("line could not be parsed").0;
        let (r,g,b) = 
                sets.iter().fold((0,0,0), 
                    | (r,g,b), s | 
                        (cmp::max(r, s.red), 
                         cmp::max(g, s.green),
                         cmp::max(b, s.blue)
                        )
                    );
        r * g * b
    }).sum();

    println!("part 2 {:?}", res);
}

