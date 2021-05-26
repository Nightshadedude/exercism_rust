#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub struct Frame {
    frame_num: usize,
    pins: u16,
    roll1: u16,
    roll2: Option<u16>,
    roll3: Option<u16>,
    frame_complete: bool,
}

impl Frame {
    pub fn new(pins: u16, frame_num: usize) -> Result<Self, Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft);  }
        if frame_num == 10 && pins == 10 {
            return Ok(Frame {
                frame_num,
                pins: 10,
                roll1: pins,
                roll2: None,
                roll3: None,
                frame_complete: false,
            });
        }

        Ok(Frame {
            frame_num,
            pins: 10-pins,
            roll1: pins,
            roll2: None,
            roll3: None,
            frame_complete: pins == 10,
        })
    }

    pub fn add_to_frame(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame_num == 10 { 
            match self.add_to_ten_frame(pins) {
                Ok(_) => { return Ok(()); },
                Err(e) => { return Err(e); },
            };
        }
        if pins > self.pins { return Err(Error::NotEnoughPinsLeft); }
        self.pins -= pins;
        self.frame_complete = true;
        Ok(self.roll2 = Some(pins))
    }

    pub fn add_to_ten_frame(&mut self, pins: u16) -> Result<(), Error> {
        dbg!("adding: {} to self: {}", pins, &self);
        if pins > self.pins {
            Err(Error::NotEnoughPinsLeft)
        } else if self.roll3.is_some() {
            Err(Error::GameComplete)
        } else if self.roll2.is_some() {
            if self.roll1 + self.roll2.unwrap() >= 10 {
                self.pins -= pins;
                self.frame_complete = true;
                Ok(self.roll3 = Some(pins))
            } else { Err(Error::GameComplete) }
        } else {
            if pins == 10 || self.roll1 + pins == 10{
                self.pins = 10;
                Ok(self.roll2 = Some(pins))
            } else if self.roll1 == 10 {
                self.pins -= pins;
                Ok(self.roll2 = Some(pins))
            } else {
                self.pins -= pins;
                self.frame_complete = true;
                Ok(self.roll2 = Some(pins))
            }
        }
    }

    pub fn is_spare(&self) -> bool {
        self.roll2.is_some() && self.pins == 0
    }

    pub fn is_strike(&self) -> bool {
        self.roll1 == 10
    }

    pub fn get_rolls(&self) -> Vec<u16> {
        let mut ret = vec![];
        ret.push(self.roll1);
        if self.roll2.is_some() { ret.push(self.roll2.unwrap()); }
        if self.roll3.is_some() { ret.push(self.roll3.unwrap()); }
        ret
    }

    pub fn frame_score(&self) -> u16 {
        self.get_rolls().iter().sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct BowlingGame {
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let len = self.frames.len();
        match len {
            0 => {
                let frame = Frame::new(pins, len + 1);
                match frame {
                    Ok(good_frame) => {
                        Ok(self.frames.push(good_frame))
                    },
                    Err(e) => Err(e),
                }
            },
            1..=9 => {
                let mut last_frame = self.frames.pop().unwrap();
                if last_frame.frame_complete {
                    self.frames.push(last_frame);
                    let frame = Frame::new(pins, len + 1);
                    match frame {
                        Ok(good_frame) => {
                            Ok(self.frames.push(good_frame))
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let frame = last_frame.add_to_frame(pins);
                    match frame {
                        Ok(_) => {
                            Ok(self.frames.push(last_frame))
                        },
                        Err(e) => Err(e),
                    }
                }
            },
            10 => {
                let mut last_frame = self.frames.pop().unwrap();
                if last_frame.frame_complete {
                    self.frames.push(last_frame);
                    Err(Error::GameComplete)
                } else {
                    let frame = last_frame.add_to_ten_frame(pins);
                    match frame {
                        Ok(_) => {
                            dbg!("ten frame add: {}", &last_frame);
                            Ok(self.frames.push(last_frame))
                        },
                        Err(Error::GameComplete) => {
                            dbg!("ten frame add: {}", &last_frame);
                            self.frames.push(last_frame);
                            Err(Error::GameComplete)
                        },
                        Err(e) => Err(e),
                    }
                }
            },
            _ => { panic!("last roll: {} - self:{:?}", pins, self); }
        }
    }

    pub fn score(&self) -> Option<u16> {
        let frames_complete = self.frames.len() == 10;
        if !frames_complete { return None; }
        let last_frame = self.frames.iter().last().unwrap().frame_complete;
        if !last_frame {return None; }
        let mut score = 0;
        let len = self.frames.len();
        for i in 0..len {
            let mut temp = i+1;
            let mut frame_score = self.frames[i].frame_score();
            let mut add_frames = 0;
            if self.frames[i].is_strike() {
                add_frames = 2;
            } else if self.frames[i].is_spare() {
                add_frames = 1;
            }
            while temp < len && add_frames > 0 {
                let next_roll = self.frames[temp].get_rolls();
                for j in 0..next_roll.len() {
                    if add_frames > 0 {
                        frame_score += next_roll[j];
                        add_frames -= 1;
                    }
                }
                temp += 1;
            }
            score += frame_score;
        }
        Some(score)
    }
}
