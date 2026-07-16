#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        // Validate pin counts for the current frame
        let mut frame = 1;
        let mut roll_idx = 0;

        while roll_idx < self.rolls.len() {
            if frame == 10 {
                // Tenth frame validation
                let rolls_left = self.rolls.len() - roll_idx;
                if rolls_left == 1 {
                    let first = self.rolls[roll_idx];
                    if first < 10 && first + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else if rolls_left == 2 {
                    let first = self.rolls[roll_idx];
                    let second = self.rolls[roll_idx + 1];
                    if first == 10 && second < 10 && second + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                break;
            }

            if self.rolls[roll_idx] == 10 {
                // Strike
                roll_idx += 1;
                frame += 1;
            } else {
                // Open frame or Spare
                if roll_idx + 1 < self.rolls.len() {
                    roll_idx += 2;
                    frame += 1;
                } else {
                    // We are in the middle of a frame
                    if self.rolls[roll_idx] + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    break;
                }
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut total_score = 0;
        let mut roll_idx = 0;

        for _frame in 1..=10 {
            if self.rolls[roll_idx] == 10 {
                // Strike
                total_score += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
                roll_idx += 1;
            } else if self.rolls[roll_idx] + self.rolls[roll_idx + 1] == 10 {
                // Spare
                total_score += 10 + self.rolls[roll_idx + 2];
                roll_idx += 2;
            } else {
                // Open Frame
                total_score += self.rolls[roll_idx] + self.rolls[roll_idx + 1];
                roll_idx += 2;
            }
        }

        Some(total_score)
    }

    fn is_complete(&self) -> bool {
        let mut frame = 1;
        let mut roll_idx = 0;

        while roll_idx < self.rolls.len() {
            if frame == 10 {
                let rolls_left = self.rolls.len() - roll_idx;
                if rolls_left < 2 {
                    return false;
                }
                let first = self.rolls[roll_idx];
                let second = self.rolls[roll_idx + 1];

                if first == 10 || first + second == 10 {
                    // Needs a third fill ball
                    return rolls_left == 3;
                }
                return rolls_left == 2;
            }

            if self.rolls[roll_idx] == 10 {
                roll_idx += 1;
                frame += 1;
            } else {
                if roll_idx + 1 < self.rolls.len() {
                    roll_idx += 2;
                    frame += 1;
                } else {
                    return false;
                }
            }
        }
        false
    }
}

