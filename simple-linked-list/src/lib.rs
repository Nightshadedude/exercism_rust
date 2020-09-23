use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<SimpleLinkedList<T>>,
}

#[derive(Debug, Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        match self.head {
            Some(node) => {},
            None => {},
        }

        len as usize
    }

    pub fn push(&mut self, element: T) {
        let mut current = self.head;
        let write_node = Node {
            data: element,
            next: None,
        };
        loop {
            match current {
                Some(mut node) => {
                    let unboxed_node = *node;
                    match unboxed_node.next {
                        Some(next_ll) => {
                            current = next_ll.head;
                        },
                        None => {
                            let write_ll = SimpleLinkedList {
                                head: Some(Box::new(write_node)),
                            };
                            unboxed_node.next = Some(write_ll);
                            break;
                        }
                    }
                },
                None => {
                   current = Some(Box::new(write_node));
                   break;
                },
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
         let mut current = self.head;
         let mut pop_data = None;
         loop {
            match current {
                Some(mut node) => {
                    let unboxed_node = *node;
                    match unboxed_node.next {
                        Some(next_ll) => {
                            current = next_ll.head;
                        },
                        None => {
                            pop_data = Some(unboxed_node.data);
                            current = None;
                            break;
                        }
                    }
                },
                None => {
                    pop_data = None;
                    break;
                },
            }
        }
        pop_data
    }

    pub fn peek(&self) -> Option<&T> {
        let node = self.head;
        match node {
            Some(n) => {
                let temp = *n;
                Some(&temp.data)
            },
            None => None,
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = SimpleLinkedList::new();
        loop {
            match self.head {
                Some(_) => ret.push(self.pop().unwrap()),
                None => break,
            }
        }
        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        for i in iter{
            ll.push(i);
        }
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut ret = vec![];
        loop {
            match self.head {
                Some(_) => ret.push(self.pop().unwrap()),
                None => break,
            }
        }
        ret.reverse();
        ret
    }
}
