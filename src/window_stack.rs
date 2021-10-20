#[derive(Debug, Clone)]
struct Node {
    val: i64,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct WindowStack {
    head: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i64) -> Self {
        Self { val, next: None }
    }

    pub fn remove(mut self, val: i64) -> Option<Box<Self>> {
        if self.val == val {
            self.next
        } else {
            Some(Box::new(
                self.next
                    .take()
                    .map(|next| Self {
                        val: self.val,
                        next: next.remove(val),
                    })
                    .unwrap_or(self),
            ))
        }
    }

    pub fn move_up(self, val: i64) -> Box<Self> {
        Box::new(Self {
            val,
            next: self.remove(val),
        })
    }

    pub fn add(self, val: i64) -> Box<Self> {
        Box::new(Self {
            val: self.val,
            next: Some(
                self.next
                    .map(|next| next.add(val))
                    .unwrap_or(Box::new(Node::new(val))),
            ),
        })
    }

    pub fn get(&self, depth: usize) -> Option<i64> {
        if depth == 0 {
            Some(self.val)
        } else {
            self.next.as_ref().and_then(|next| next.get(depth - 1))
        }
    }

    pub fn depth(&self) -> usize {
        self.next.as_ref().map(|next| next.depth()).unwrap_or(0) + 1
    }
}

impl WindowStack {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn move_up(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.move_up(id))
            .or(Some(Box::new(Node::new(id))));
        id
    }

    pub fn add(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.add(id))
            .or(Some(Box::new(Node::new(id))));
        id
    }

    pub fn remove(&mut self, id: i64) -> i64 {
        self.head = self.head.take().and_then(|head| head.remove(id));
        id
    }

    pub fn get(&self, depth: usize) -> Option<i64> {
        self.head.as_ref().and_then(|head| head.get(depth))
    }

    pub fn depth(&self) -> usize {
        self.head.as_ref().map(|head| head.depth()).unwrap_or(0)
    }
}
