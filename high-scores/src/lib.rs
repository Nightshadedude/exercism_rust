#[derive(Debug)]
pub struct HighScores {
    high_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            high_scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.high_scores
    }

    pub fn latest(&self) -> Option<u32> {
        let length = &self.high_scores.len(); 
        match length {
            0 => None,
            _ => Some(self.high_scores[length-1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match &self.high_scores.len() {
            0 => None,
            _ => {
                let mut sorted = self.high_scores.clone();
                sorted.sort();
                sorted.reverse();
                Some(sorted[0])
            },
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
         
        let length = &self.high_scores.len();
        let mut sorted = self.high_scores.clone();
        sorted.sort();
        sorted.reverse();
        match length {
            0..=3 => sorted,
            _ => sorted[0..3].to_vec(),
        }
    }
}
