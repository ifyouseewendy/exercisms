#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().map(|v| *v)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut v = self.0.to_owned();
        v.sort();
        v.last().map(|v| *v)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.0.to_owned();
        v.sort();

        let mut ret = vec![];
        while let Some(e) = v.pop() {
            ret.push(e);
            if ret.len() == 3 { break }
        }
        ret
    }
}
