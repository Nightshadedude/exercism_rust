pub struct School {
    grades: Vec<(u32,Vec<String>)>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: vec![],
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let mut pushed = false;
        for i in 0..self.grades.len() {
            if self.grades[i].0 == grade {
                self.grades[i].1.push(String::from(student));
                pushed = true;
            }
        }
        if !pushed {
            self.grades.push((grade, vec![String::from(student)]));
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades.iter().map(|(gr, _)| *gr).collect::<Vec<u32>>();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut students = vec![];
        for i in 0..self.grades.len() {
            if self.grades[i].0 == grade {
                students = self.grades[i].clone().1;
            }
        }
        students.sort();
        match students.len() {
            0 => return None,
            _ => return Some(students),
        }
    }
}
