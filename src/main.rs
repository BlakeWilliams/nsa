extern crate getopts;
extern crate rand;
extern crate regex;

use getopts::Options;
use rand::distributions::{IndependentSample, Range};
use regex::Regex;
use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("c", "count", "how many words to print", "COUNT");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let word_count = if matches.opt_present("c") {
        let count_string = matches.opt_str("c").unwrap();

        count_string.parse::<u32>()
            .ok()
            .expect(&format!("Could not coerce '{}' into number", count_string))
    } else {
        15
    };

    let words = read_words();
    let between = Range::new(0, words.len());
    let mut rng = rand::thread_rng();

    let phrase: Vec<String> = (0..word_count).map(|_i| {
        let index = between.ind_sample(&mut rng);
        words.get(index).unwrap().to_string()
    }).collect();

    println!("{}", phrase.connect(" "));
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn read_words() -> Vec<String> {
    let mut words= String::new();
    let home = env::home_dir()
        .expect("Could not get home directory");
    let nsa_path = home.to_str().expect("").to_string() + "/.nsa";

    let mut nsa_file = File::open(nsa_path)
        .ok()
        .expect("Could not read ~/.nsa");

    File::read_to_string(&mut nsa_file, &mut words)
        .ok()
        .expect("Could not read ~/.nsa");

    let re = Regex::new(r"\r?\n").unwrap();
    re.split(&words).map(|c| c.to_string() ).collect()
}
