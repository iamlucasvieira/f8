use rand::Rng;

const ANSWERS: &str = include_str!("answers.txt");

pub struct Oracle {
    answers: Vec<String>,
}

impl Oracle {
    pub fn new() -> Self {
        let answers = ANSWERS.lines().map(|s| s.to_string()).collect();
        Self { answers }
    }

    pub fn answer(&self) -> &str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.answers.len());
        &self.answers[index]
    }
}
