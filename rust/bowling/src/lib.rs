#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
enum State {
    Init,
    Pending,
    Bonus,
    Done,
}

#[derive(Debug, PartialEq)]
enum FrameResult {
    Strike,
    Spare,
    Open,
}

#[derive(Debug, PartialEq)]
struct Frame {
    result: FrameResult,
    first: u16,
    second: u16,
}

impl Frame {
    pub fn from(first: u16, second: u16) -> Result<Self, Error> {
        if first + second > 10 { return Err(Error::NotEnoughPinsLeft); }

        let result = if first == 10 {
            FrameResult::Strike
        } else if first + second == 10 {
            FrameResult::Spare
        } else {
            FrameResult::Open
        };

        Ok(Self { first, second, result })
    }
    pub fn sum(&self) -> u16 {
        self.first + self.second
    }
}

pub struct BowlingGame {
    scores: Vec<Frame>,
    state: State,
    last_throw: u16,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            scores: Vec::with_capacity(12),
            state: State::Init,
            last_throw: 0,
        }
    }
}
impl BowlingGame {
    pub fn new() -> Self { Default::default() }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }

        match self.state {
            State::Init => {
                if pins == 10 {
                    self.scores.push(Frame::from(10, 0)?);
                    if self.scores.len() < 10 { return Ok(()); }
                }

                self.last_throw = pins;
                self.state = State::Pending
            },
            State::Pending => {
                if self.scores.len() < 9 {
                    self.scores.push(Frame::from(self.last_throw, pins)?);
                    self.state = State::Init;
                } else if self.last_throw == 10 {
                    if pins == 10 { self.scores.push(Frame::from(10, 0)?); }
                    self.last_throw = pins;
                    self.state = State::Bonus;
                } else {
                    self.scores.push(Frame::from(self.last_throw, pins)?);

                    if self.last_throw + pins == 10 {
                        self.last_throw = 0;
                        self.state = State::Bonus;
                    } else {
                        self.state = State::Done;
                    }
                }
            },
            State::Bonus => {
                if self.last_throw == 10 {
                    self.scores.push(Frame::from(pins, 0)?);
                } else {
                    self.scores.push(Frame::from(pins, self.last_throw)?);
                }
                self.state = State::Done;
            },
            State::Done => return Err(Error::GameComplete)
        }

        if self.state == State::Done {
            while self.scores.len() < 12 { self.scores.push(Frame::from(0, 0)?); }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.state != State::Done { return None }

        Some((0..10).fold(0, |total, i| {
            let current = &self.scores[i];
            let next    = &self.scores[i+1];
            let bonus   = &self.scores[i+2];

            match current.result {
                FrameResult::Strike => {
                    if next.result == FrameResult::Strike {
                        total + current.sum() + next.sum() + bonus.first
                    } else {
                        total + current.sum() + next.sum()
                    }
                },
                FrameResult::Spare => {
                    total + current.sum() + next.first
                },
                FrameResult::Open => {
                    total + current.sum()
                },
            }
        }))
    }
}
