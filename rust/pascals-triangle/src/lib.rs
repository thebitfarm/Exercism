
pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {

    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{ row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut ret:Vec<Vec<u32>> = vec![];

        for r in 0..self.row_count {
            println!("On row {}", r);
            let mut curr = vec![];

            //if r > 0 {
                for i in 0..(1+r) {
                    curr.push( binom(r, i) );
                    println!("[{}][{}] = {}", r, i, curr[i as usize]);
                }
                ret.push( curr );
            //}
        }

        ret
    }


}

fn binom(n: u32, k: u32) -> u32 {
    let mut ret = 1u32;
    for i in 0..k {
        ret = (ret * (n - i)) /
                (i + 1);
    }
    ret
}