extern crate num;
use num::Num;

#[derive(Debug)]
pub struct Triangle<T: Num + Copy + PartialEq + Ord + Clone + PartialOrd> {
    sides : Vec<T>
}



impl<T: Num + Copy + PartialEq + Ord + Clone + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut tmparr : Vec<T> = sides.to_vec();
        tmparr.sort();

        // First -> last -> outer
        match (tmparr.get(0), tmparr.get(1), tmparr.get(2) ) {
            (Some(a), Some(b), Some(c)) => match (*a+*b) > *c {
                false => None,
                true => match (*b + *c) > *a {
                    false => None,
                    true => match (*a + *c) > *b {
                        false => None,
                        true => Some( Triangle{sides: tmparr.clone(),} )
                    }
                }
            },
            _ => None
        }

    }

    pub fn is_equilateral(&self) -> bool {
        match (self.sides.get(0), self.sides.get(1), self.sides.get(2) ) {
            (Some(a), Some(b), Some(c)) => (*a==*b && *a==*c),
            _ => false
        }
    }

    pub fn is_scalene(&self) -> bool {
        match (self.sides.get(0), self.sides.get(1), self.sides.get(2) ) {
            (Some(a), Some(b), Some(c)) => (*a!=*b && *a!=*c && *b!=*c),
            _ => false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        match (self.sides.get(0), self.sides.get(1), self.sides.get(2) ) {
            (Some(a), Some(b), Some(c)) => (*a==*b || *b==*c),
            _ => false
        }
    }
}
