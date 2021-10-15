use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i64,
    next: Option<Rc<Node>>,
}

#[derive(Debug)]
pub struct WindowStack {
    head: Option<Rc<Node>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            val: self.val,
            next: match &self.next {
                Some(v) => {
                    let next = v.as_ref().clone();
                    Some(Rc::from(next))
                }
                None => None,
            },
        }
    }
}

impl Node {
    pub fn new(val: i64) -> Self {
        Self { val, next: None }
    }

    pub fn remove(self, val: i64) -> Option<Rc<Self>> {
        if self.val == val {
            self.next
        } else {
            Some(Rc::from(match self.next {
                Some(v) => Self {
                    val: self.val,
                    next: v.as_ref().clone().remove(val),
                },
                None => self,
            }))
        }
    }

    pub fn move_up(self, val: i64) -> Rc<Self> {
        if let Some(s) = self.remove(val) {
            Rc::from(Self { val, next: Some(s) })
        } else {
            Rc::from(Self { val, next: None })
        }
    }

    pub fn add(self, val: i64) -> Rc<Self> {
        if let Some(v) = self.next {
            let next = v.as_ref().clone().add(val);
            Rc::from(Self {
                val: self.val,
                next: Some(next),
            })
        } else {
            let node = Node::new(val);
            let next = Some(Rc::from(node));
            Rc::from(Self {
                val: self.val,
                next,
            })
        }
    }

    pub fn get(&self, depth: usize) -> Option<i64> {
        if depth == 0 {
            Some(self.val)
        } else if let Some(v) = &self.next {
            v.get(depth - 1)
        } else {
            None
        }
    }

    pub fn depth(&self) -> usize {
        if let Some(v) = &self.next {
            v.depth() + 1
        } else {
            1
        }
    }
}

impl Clone for WindowStack {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
        }
    }
}

impl WindowStack {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn move_up(&mut self, id: i64) -> i64 {
        if let Some(v) = &self.head {
            self.head = Some(v.as_ref().clone().move_up(id));
        } else {
            let node = Node::new(id);
            let rc = Rc::from(node);
            self.head = Some(rc);
        }
        id
    }

    pub fn add(&mut self, id: i64) -> i64 {
        if let Some(v) = &self.head {
            self.head = Some(v.as_ref().clone().add(id));
        } else {
            self.head = Some(Rc::from(Node::new(id)));
        }
        id
    }

    pub fn remove(&mut self, id: i64) -> i64 {
        if let Some(v) = &self.head {
            self.head = v.as_ref().clone().remove(id);
        }
        id
    }

    pub fn get(&self, depth: usize) -> Option<i64> {
        if let Some(v) = &self.head {
            v.get(depth)
        } else {
            None
        }
    }

    pub fn depth(&self) -> usize {
        if let Some(v) = &self.head {
            v.depth()
        } else {
            0
        }
    }
}
