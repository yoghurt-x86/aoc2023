use std::collections::HashMap;
use std::fs;

extern crate combine;
use combine::{many1, Parser, many};
use combine::parser::char::{spaces, space, char, alpha_num, newline};


fn main() {
    let dirs = || many1(char('L').or(char('R')))
            .skip(newline())
            .skip(spaces())
            .map(|s :String| s.chars().collect::<Vec<char>>());

    let word = || many1(alpha_num()).map(|s: String| s);

    let line = || word()
        .skip(space())
        .skip(char('='))
        .skip(space())
        .skip(char('('))
        .and(word())
        .skip(char(','))
        .skip(space())
        .and(word())
        .skip(char(')'))
        .skip(newline())
        .map(|((w1,w2), w3)| (w1,w2,w3));

    let file = || dirs().and(many(line()));

    let contents = fs::read_to_string("./08.input")
        .expect("Should have been able to read the file");

    let (directions, words) : (Vec<_>, Vec<_>)= file().parse(contents.as_str()).unwrap().0;
    let mut map : HashMap<&str, (&str, &str)>= HashMap::new();
    let mut starting_keys : Vec<&str> = Vec::new();
    words.iter().for_each(|(w1,w2,w3)| { 
            map.insert(w1.as_str(), (w2.as_str(),w3.as_str())); 
            if w1.ends_with('A'){
                starting_keys.push(w1.as_str());
            }
        });
    let mut counts : Vec<usize> = vec![0;starting_keys.len()];

    println!("{:?}", starting_keys);
    let mut i : usize = 0;
    let mut forks : Vec<(_,_)> = starting_keys.iter().map(|k| map.get(*k).unwrap().clone()).collect();
    loop {
        let dir : char = directions[i % directions.len()];
        i = i + 1;
        let keys : Vec<&str> = forks.iter().map(|(left, right)| if dir == 'L' {left.clone()} else {right.clone()}).collect();
        keys.iter().enumerate().for_each(|(j,k)| {
            if k.ends_with('Z') {
                counts[j] = i;
            }
        });
        forks = keys.iter().map(|k| map.get(k).unwrap().clone()).collect();
        println!("{:?}", keys);

        if counts.iter().all(|c| *c != 0 ) {
            println!("{:?}", keys)
        } else {
            forks = keys.iter().map(|k| map.get(k).unwrap().clone()).collect();
        }
        if 1 == 0{
            break;
        }
    }

    println!("part 1: {:?}", words);
    println!("part 1: {:?}", map);
    println!("part 1: {:?}", counts);
    println!("part 1: {:?}", i);
}
