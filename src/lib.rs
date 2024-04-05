#[derive(Debug)]
pub struct Triangle {
    x1: u64,
    x2: u64,
    x3: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let x1 = sides[0];
        let x2 = sides[1];
        let x3 = sides[2];

        
        if x1 > 0 && x2 > 0 && x3 > 0 && x1 + x2 >= x3 && x1 + x3 >= x2 && x2 + x3 >= x1 {
            Some(Triangle { x1, x2, x3 })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.x1 == self.x2 && self.x2 == self.x3
    }

    pub fn is_scalene(&self) -> bool {
        self.x1 != self.x2 && self.x2 != self.x3 && self.x3 != self.x1
    }

    pub fn is_isosceles(&self) -> bool {
        self.x1 == self.x2 || self.x1 == self.x3 || self.x2 == self.x3
    }
}
    

