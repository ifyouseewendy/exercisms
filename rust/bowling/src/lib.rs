#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
enum State {
    Init,
    Pending,
    Done,
}

pub struct BowlingGame {
    scores: [(u16, u16); 10],
    frame: usize,
    state: State,
    temp: u16,
    temp2: u16,
    bonus: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scores: [(0, 0); 10],
            frame: 0,
            state: State::Init,
            temp: 0,
            temp2: 99,
            bonus: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }

        match self.state {
            State::Init => {
                if pins == 10 {
                    if self.frame == 9 {
                        self.temp = pins;
                        self.state = State::Pending;
                    } else {
                        self.scores[self.frame] = (10, 0);
                        self.frame += 1;
                    }
                } else {
                    self.temp = pins;
                    self.state = State::Pending;
                }
            },
            State::Pending => {
                if self.frame == 9 {
                    if self.temp2 == 99 {
                        if self.temp == 10 || self.temp + pins == 10 {
                            self.temp2 = pins;
                        } else {
                            self.scores[self.frame] = (self.temp, pins);
                            self.state = State::Done;
                        }
                    } else {
                        if self.temp == 10 && self.temp2 != 10 && self.temp2 + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }

                        self.scores[self.frame] = (self.temp, self.temp2);
                        self.bonus = pins;
                        self.state = State::Done;
                    }
                } else {
                    if self.temp + pins > 10 { return Err(Error::NotEnoughPinsLeft) }

                    self.scores[self.frame] = (self.temp, pins);
                    self.frame += 1;
                    self.state = State::Init;
                }
            },
            State::Done => return Err(Error::GameComplete)
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.state != State::Done { return None }

        let mut total = 0;
        self.scores.windows(3).for_each(|w| {
            if w[0] == (10, 0) { //strike
                if w[1] == (10, 0) { //strike again
                    total += 10 + 10 + w[2].0;
                } else {
                    total += 10 + w[1].0 + w[1].1;
                }
            } else if w[0].0 + w[0].1 == 10 {
                total += 10 + w[1].0;
            } else {
                total += w[0].0 + w[0].1;
            }
        });

        println!("{:?}", self.scores);
        let a = self.scores[8];

        if a == (10, 0) {
            total += 10 + self.scores[9].0 + self.scores[9].1;
        } else if a.0 + a.1 == 10 {
            total += 10 + self.scores[9].0;
        } else {
            total += a.0 + a.1;
        }

        total += self.scores[9].0 + self.scores[9].1 + self.bonus;

        Some(total)
    }
}
