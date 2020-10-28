pub struct Triangle{
    side: Vec<u64>,
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        let mut ret_sides = vec![];
        let mut sum = 0;
        sides.sort();
        for i in 0..sides.len() {
            if sides[i] <= 0 { return None; }
            else {
                if i == 2 && sides[i] >= sum { return None; }
                sum += sides[i];
                ret_sides.push(sides[i]);
            }
        }

        Some(Triangle{
            side: ret_sides,
        })
    }

    pub fn is_equilateral(&self) -> bool {
        if self.side[0] == self.side[1] && self.side[1] == self.side[2] { return true; }
        false
    }

    pub fn is_scalene(&self) -> bool {
        if self.side[0] != self.side[1]
            && self.side[1] != self.side[2]
            && self.side[2] != self.side[0]
            { return true; }
        false
    }

    pub fn is_isosceles(&self) -> bool {
         if self.side[0] == self.side[1]
            || self.side[1] == self.side[2]
            || self.side[2] == self.side[0]
            { return true; }
        false
    }
}
