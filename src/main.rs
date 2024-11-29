use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = std::env::args().skip(1);

    //let puzzle = wordf::generator::PuzzleGenerator::generate(
    //    args.next().expect("A string to generate"),
    //    (
    //            args.next()
    //                .unwrap_or(20.to_string())
    //                .parse::<i32>().expect("Integer"),
    //            args.next()
    //                .unwrap_or(20.to_string())
    //                .parse::<i32>().expect("Integer"),
    //    ),
    //);
    //println!("{}", puzzle.to_string());

    let filename = args.next().unwrap();
    let target = args.next().unwrap();

    let mut file = File::open(filename).unwrap();

    let puzzle = wordf::types::Puzzle {
        chars: {let mut string = String::new(); file.read_to_string(&mut string).unwrap(); string.lines().map(|l| l.chars().collect::<Vec<_>>()).collect()},
        target
    };

    let solution = wordf::solver::Solver::solve(&puzzle);
    println!("{:?}", solution);
}
