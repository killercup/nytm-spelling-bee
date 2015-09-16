use std::io::prelude::*;
use std::{env, io, fs};
use std::collections::BTreeSet;

const WORDS_FILE : &'static str = "/usr/share/dict/words";
type Letters = u32;
const NONE : Letters = 0;
const Z : Letters = 1;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from(WORDS_FILE));
    let stdin = io::stdin();
    let file : Box<io::Read> = match &*name {
        "-" => Box::new(stdin.lock()),
        _   => Box::new(fs::File::open(name).ok().expect("file open failed"))
    };

    let mut words : Vec<Letters> = Vec::new();
    let sevens : BTreeSet<_> = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() >= 5)
        .map(|line| line.bytes()
            .scan((0, NONE), |pair, c| {
                if (*pair).0 <= 7 {
                    let new = match c as char {
                        'a' ... 'z' => (*pair).1 | Z << ('z' as u8) - c,
                        _  => !NONE
                    };
                    *pair = (new.count_ones(), new);
                    Some(*pair)
                } else { None }
            }).last().unwrap())
        .filter(|&pair| pair.0 <= 7)
        .inspect(|&pair| words.push(pair.1))
        .filter(|&pair| pair.0 == 7)
        .map(|pair| pair.1)
        .collect();

    let stdout = io::stdout();
    let mut sink = io::BufWriter::new(stdout.lock());
    sevens.iter().rev().map(|&seven| {
        let scores = words.iter()
            .filter(|&&word| word & !seven == 0)
            .map(|&word| (word, if word == seven { 3 } else { 1 }))
            .fold([0;7], |mut scores, (word, points)| {
                scores.iter_mut().fold(seven, |rest, score| {
                    if word & rest & !(rest - 1) != 0 {
                        *score += points }
                    rest & rest - 1
                });
                scores
            });
        let mut out = [0, 0, 0, 0, 0, 0, 0, '\n' as u8];
        let (is_viable, _) = scores.iter().zip(out.iter_mut().rev().skip(1))
            .fold((false, seven), |(mut is_viable, rest), (&score, out)| {
                let z = match score {
                    26 ... 32 => { is_viable = true; 'Z' as u8 },
                    _         => 'z' as u8
                };
                *out = z - (rest.trailing_zeros() as u8);
                (is_viable, rest & rest - 1)
            });
         if is_viable {
              sink.write(&out).unwrap(); };
    }).count();
}
