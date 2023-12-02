use std::collections::HashMap;
use std::fs;

extern crate combine;
use combine::{choice, many1, Parser, sep_by};
use combine::parser::char::{spaces, space, string, char, digit, alpha_num, newline};
use itertools::Itertools;
use std::cmp;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn count_cards(cards: &str) -> Kind {
    let mut map : HashMap<char, i32>= HashMap::new();
    for c in cards.chars() {
        map.insert(c, map.get(&c).unwrap_or(&0) + 1);
    }
    let mut f = map.values().sorted().collect::<Vec<_>>();
    f.reverse();
    match f[0] {
        &5 => Kind::Five,
        &4 => Kind::Four,
        &3 => { if f[1] == &2 { Kind::FullHouse } else { Kind::Three} },
        &2 => { if f[1] == &2 { Kind::TwoPair } else { Kind::OnePair} },
        &1 => Kind::HighCard,
        _  => panic!("wtf")
    }
}

fn card_order(c: char) -> i32 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card"),
    }
}

fn cmp_hands(a: (&str, Kind), b: (&str, Kind)) -> std::cmp::Ordering {
    match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => {
            for (ac, bc) in a.0.chars().zip(b.0.chars()) {
                match card_order(ac).cmp(&card_order(bc)) {
                    std::cmp::Ordering::Equal => (),
                    inequal => return inequal,
                }
            }
            std::cmp::Ordering::Equal
        },
        other => other,

    }
}


fn main() {
    //let lex_char = |c| char(c).skip(spaces());

    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let word = || many1(alpha_num()).map(|s: String| s);
    let line = || (word().skip(space()), integer().skip(newline()))   
            .map(|(card, bid) : (String,  i32)| (card.clone(), count_cards(&card), bid));
    let file = || many1(line());
        
    let contents = fs::read_to_string("./07.input")
        .expect("Should have been able to read the file");

    let mut res : Vec<_> =  file().parse(contents.as_str()).unwrap().0;

    res.sort_by(|a, b| cmp_hands((&a.0, a.1), (&b.0, b.1)));

    let sum : u64 = res.iter().enumerate().map(|(i, (_,_,bid))| 
        u64::try_from(i + 1).unwrap() * u64::try_from(*bid).unwrap()
    ).sum();


    println!("part 1: {}", sum);
}
