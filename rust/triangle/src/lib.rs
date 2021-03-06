pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides;
        sides.sort();
        let [a, b, c] = sides;

        if a == 0 || a + b < c { None }
        else { Some(Self(a, b, c)) }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2
    }
   
    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2
    }
}
