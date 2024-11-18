fn main() {
    let mut args = std::env::args().skip(1);
    let puzzle = wordf::generator::PuzzleGenerator::generate(
        args.next().expect("A string to generate"),
        (
                args.next()
                    .unwrap_or(20.to_string())
                    .parse::<i32>().expect("Integer"),
                args.next()
                    .unwrap_or(20.to_string())
                    .parse::<i32>().expect("Integer"),
        ),
    );
    println!("{}", puzzle.to_string());
}
