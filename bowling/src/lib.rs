#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    FrameFull,
}

pub struct Frame {
    roll_1: u16,
    roll_2: Option<u16>,
}

impl Frame {
    pub fn new(pins: u16) -> Result<Self, Error> {
        match pins {
            0..=9 => {
                Ok(Frame {
                    roll_1: pins,
                    roll_2: None,
                }),
            },
            10 => Ok(Frame {
                roll_1: pins,
                roll_2: Some(0),
            },
            _ => Err(Error::NotEnoughPinsLeft),
    }

    pub fn update(&mut self, pins: u16) -> Result<(), Error> {
        match self.roll_2 {
            Some(_) => Err(Error::FrameFull),
            None => {
                match self.roll_1 + pins {
                    0..=10 => {
                        self.roll_2 = Some(pins);
                        Ok(())
                    },
                    _ => Err(Error::NotEnoughPinsLeft),
                }
            },
        }
    }
}

pub struct BowlingGame {
   frames: Vec<Frame>,
   ten_frame: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
            ten_frame: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let len = self.frames.len();
        match len {
            0 => {
               match pins {
                   0..=9 => {
                       let frame = Frame {
                           roll_1: pins,
                           roll_2: None,
                       };
                       self.frames.push(frame);
                       Ok(())
                   },
                   10 => {
                       let frame = Frame {
                           roll_1: pins,
                           roll_2: Some(0),
                       };
                       self.frames.push(frame);
                       Ok(())
                   },
                   _ => Err(Error::NotEnoughPinsLeft),
               }
            },
            1..=9 {
                match self.frames.last().unwrap().update(pins) {
                    Ok => Ok(()),
                    Err(Error::FrameFull) => {
                        match pins {
                            0..=9
            },
            10 => {
            },
            _ => Err(GameComplete),
        }
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
