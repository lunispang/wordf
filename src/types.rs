pub struct Puzzle {
    pub chars: Vec<Vec<char>>,
    pub target: String,
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        self.chars.iter().map(|s| String::from_iter(s)).collect::<Vec<String>>().join("\n")
    }
}

pub struct Solution {
    seq: Vec<(i32, i32)>
}
