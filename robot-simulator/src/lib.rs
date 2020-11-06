// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    dir: Direction,
    point: (i32, i32),
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            dir: d,
            point: (x,y),
        }
    }

    pub fn turn_right(self) -> Self {
        let pt = self.point;
        let dir = match self.dir {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };

        Robot {
            dir,
            point: pt,
        }
    }

    pub fn turn_left(self) -> Self {
        let pt = self.point;
        let dir = match self.dir {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        };

        Robot {
            dir,
            point: pt,
        }
    }

    pub fn advance(self) -> Self {
        let (mut x, mut y) = self.point;
        match self.dir {
            Direction::North => {
                y = y + 1;
            },
            Direction::South => {
                y = y - 1;
            },
            Direction::East => {
                x = x + 1;
            },
            Direction::West => {
                x = x - 1;
            },
        }

        Robot {
            dir: self.dir,
            point: (x, y),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut bot = self;
        for ch in instructions.chars().into_iter() {
            match ch {
                'A' => bot = bot.advance(),
                'R' => bot = bot.turn_right(),
                'L' => bot = bot.turn_left(),
                _ => bot = bot,
            }
        }
        bot
    }

    pub fn position(&self) -> (i32, i32) {
        self.point
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
