
extern crate itertools;

#[derive(Debug, PartialEq)]
pub struct DNA {
    dna : Vec<char>
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna : Vec<char>
}

impl DNA {


    pub fn check_nucleotide(nucleotide: char) -> bool {
        match nucleotide {
            'A' | 'C' | 'G' | 'T' => true,
            _ => false 
        }
    }


    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut res : Vec<char> = vec![];

        for( i, nucleotide ) in dna.chars().enumerate() {
            match DNA::check_nucleotide(nucleotide) {
                true => res.push(nucleotide),
                false => return Err(i)
            };

        }

        Ok(DNA{ dna: res })
    }

    pub fn into_rna(self) -> RNA {
        let full : String = self.dna.into_iter().map(|c| {
            match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => 'X'
            }
        }).collect::<String>();
        RNA::new( full.as_str() ).unwrap()
    }
}

impl RNA {


    fn check_nucleotide(nucleotide: char) -> bool {
        match nucleotide {
            'A' | 'C' | 'G' | 'U' => true,
            _ => false 
        }
    }

    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut res : Vec<char> = vec![];

        for( i, nucleotide ) in rna.chars().enumerate() {
            match RNA::check_nucleotide(nucleotide) {
                true => res.push(nucleotide),
                false => return Err(i)
            };

        }

        Ok(RNA{ rna: res })
    }
}



