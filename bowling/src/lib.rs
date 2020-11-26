#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    FrameFull,
}

pub struct Frame {
    roll: Vec<u16>,
}

pub struct BowlingGame {
   frames: Vec<Frame>,
}


impl Frame {
    pub fn new(pins: u16) -> Result<Self, Error> {
        match pins {
            0..=9 => Ok(Frame {
                roll: vec![pins],
            }),
            10 => Ok(Frame {
                roll: vec![pins, 0],
            }),
            _ => Err(Error::NotEnoughPinsLeft),
        }
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
                let frame = Frame::new(pins);
                match frame {
                }
            },
            1..=9 => {
            },
            10 => {
            },
            _ => Err(Error::GameComplete),
        }
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
