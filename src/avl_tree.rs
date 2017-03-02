use std::cmp;
use std::fmt;
use std::fmt::{Display, Debug};

trait Balancer {
    fn balance_all(&mut self);
    fn small_right_turn(&mut self);
    fn small_left_turn(&mut self);
    fn big_right_turn(&mut self);
    fn big_left_turn(&mut self);
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Display for Node<T>
    where T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n", self.value);
        if let Some(ref left) = self.left {
            write!(f, "\tleft = {}", *left);
        } else {
            write!(f, "\tno left\n");
        }
        if let Some(ref right) = self.right {
            write!(f, "\tright = {}", *right);
        } else {
            write!(f, "\tno right\n");
        }
        Ok(())
    }
}

impl<T> Node<T>
    where T: Ord
{
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn left_height(&self) -> i64 {
        match self.left {
            Some(ref x) => x.height(),
            None => -1,
        }
    }

    fn right_height(&self) -> i64 {
        match self.right {
            Some(ref x) => x.height(),
            None => -1,
        }
    }

    pub fn height(&self) -> i64 {
        1 + cmp::max(self.left_height(), self.right_height())
    }

    pub fn vertex_balance(&self) -> i64 {
        self.left_height() - self.right_height()
    }

    pub fn insert_unbalanced(&mut self, value: T) {
        if self.value == value {
            return;
        }
        let target_node = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert_unbalanced(value),
            &mut None => {
                let new_node = Node {
                    value: value,
                    left: None,
                    right: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    fn find(&self, value: T) -> Option<&Node<T>> {
        if self.value == value {
            Some(self)
        } else if value < self.value {
            if let Some(ref left) = self.left {
                left.find(value)
            } else {
                None
            }
        } else {
            if let Some(ref right) = self.right {
                right.find(value)
            } else {
                None
            }
        }
    }

    fn delete(&mut self, value: T) {
        if let Some(node) = self.find(value) {

        }
    }
}

impl<T> Balancer for Node<T>
    where T: Ord
{
    fn balance_all(&mut self) {
        match self.vertex_balance() {
            -1...1 => {
                return;
            }
            2 => {
                match self.left.unwrap().vertex_balance() {
                    0 | 1 => {
                        self.small_right_turn();
                    }
                    -1 => {
                        self.big_right_turn();
                    }
                    _ => {
                        if self.left.is_some() {
                            self.left.unwrap().balance_all();
                        };
                        if self.right.is_some() {
                            &self.right.unwrap().balance_all();
                        };
                    }
                }
            }
            -2 => {
                match self.right.unwrap().vertex_balance() {
                    0 | -1 => {
                        self.small_left_turn();
                    }
                    1 => {
                        self.big_left_turn();
                    }
                    _ => {
                        if self.left.is_some() {
                            self.left.unwrap().balance_all();
                        };
                        if self.right.is_some() {
                            self.right.unwrap().balance_all();
                        };
                    }
                }
            }
            _ => {
                if self.left.is_some() {
                    self.left.unwrap().balance_all();
                };
                if self.right.is_some() {
                    self.right.unwrap().balance_all();
                };
            }
        }
    }

    fn small_right_turn(&mut self) {
        if let Some(ref left) = self.left {
            let ref alpha = left.left;
            let ref gamma = left.right;
            let ref delta = self.right;
            let B = self;
            let ref A = *left;
            self = &mut *A;
            self.left = *alpha;
            self.right = Some(Box::new(*B));
            B.left = *gamma;
            B.right = *delta;
        }
    }
    fn small_left_turn(&mut self) {}
    fn big_right_turn(&mut self) {}
    fn big_left_turn(&mut self) {}
}
