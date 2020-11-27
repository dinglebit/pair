use rand::seq::SliceRandom;
use rand::thread_rng;

use std::io::{self, BufRead};

fn main() {
    // read from stdin
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let mut pairs: Vec<String> = vec![];
    let mut same = true;
    while same {
        // generate the randomized list
        pairs = lines.clone();
        let mut rng = thread_rng();
        pairs.shuffle(&mut rng);

        // print out the pairs.
        same = false;
        for (x, name) in lines.iter().enumerate() {
            if *name == pairs[x] {
                same = true;
                break;
            }
        }
    }

    // print out the pairs.
    for (x, name) in lines.iter().enumerate() {
        println!("{} => {}", name, pairs[x]);
    }
}
