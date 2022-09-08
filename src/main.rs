use std::process;

use clap::Parser;
use rand::seq::SliceRandom;
use squirrel_rng::SquirrelRng;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("name list must contain at least 2 names (contains {0})")]
    MinimumLength(usize),
}

#[derive(Debug, Parser)]
struct Args {
    names: Vec<String>,

    #[clap(short, long)]
    bruce: bool,
}

impl Args {
    fn names(&self) -> Result<impl Iterator<Item = &str>, Error> {
        if self.names.len() < 2 {
            return Err(Error::MinimumLength(self.names.len()));
        }

        Ok(self.names.iter().map(AsRef::as_ref))
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Error> {
    let names: Vec<_> = args.names()?.collect();

    let mut pairings = shuffle(&names);
    pairings.sort_by(|a, b| a.0.cmp(b.0));

    for (left, right) in pairings {
        if args.bruce && left.to_ascii_uppercase() == "ARCHER" {
            println!("{left} -> Bruce Campbell");
        } else {
            println!("{left} -> {right}");
        }
    }

    Ok(())
}

fn shuffle<'a>(names: &'a [&str]) -> Vec<(&'a str, &'a str)> {
    if names.len() == 2 {
        return names
            .iter()
            .copied()
            .zip(names.iter().copied().rev())
            .collect();
    }

    let mut rng = SquirrelRng::new();
    let mut rhs: Vec<_> = names.to_vec();

    // I have no chill. I swear I was going to do this in a nicer way, but the technical term for
    // this strategy is "screw it."

    loop {
        rhs.shuffle(&mut rng);
        let candidate = combine(names, &rhs);
        if is_valid(&candidate) {
            return candidate;
        }
    }
}

fn is_valid(pairings: &[(&str, &str)]) -> bool {
    pairings.iter().all(|&(left, right)| left != right)
}

fn combine<'a>(left: &[&'a str], right: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    left.iter().copied().zip(right.iter().copied()).collect()
}
