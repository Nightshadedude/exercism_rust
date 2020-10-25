#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if dna.len() == 0 { return Err(0usize)}
        for i in 0..dna.len() {
            let ch = dna.chars().nth(i).unwrap();
            match ch {
                'A' => (),
                'C' => (),
                'G' => (),
                'T' => (),
                _ => return Err(i),
            }
        }
        Ok(DNA{
            dna: String::from(dna),
        })
    }


    pub fn into_rna(self) -> RNA {
        let ret_str = self.dna
            .chars()
            .map(|ch| match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => ' ',
            }).collect::<Vec<char>>()
            .into_iter()
            .collect::<String>();
        RNA::new(&ret_str).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if rna.len() == 0 { return Err(0usize)}
        for i in 0..rna.len() {
            let ch = rna.chars().nth(i).unwrap();
            match ch {
                'U' => (),
                'G' => (),
                'C' => (),
                'A' => (),
                _ => return Err(i),
            }
        }
        Ok(RNA{
            rna: String::from(rna),
        })
    }

}
