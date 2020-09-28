#[derive(Debug, Clone)]
pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut tri_builder = vec![];
        let mut last_vec = vec![];
        for i in 1..=row_count {
            match i {
                1 => last_vec = vec![1],
                2 => last_vec = vec![1,1],
                _ => {
                    let mut temp = vec![];
                    let mut last_elem = 0;
                    for elem in last_vec.clone().iter() {
                        temp.push(elem+last_elem);
                        last_elem = *elem;
                    }
                    temp.push(1);
                    last_vec = temp;
                }
            }
            tri_builder.push(last_vec.clone());
        }
        PascalsTriangle{
            triangle: tri_builder,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
