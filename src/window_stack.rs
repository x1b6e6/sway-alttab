/// `Node` is internal type for storing data (of [`i64`]) in [`WindowStack`]
#[derive(Debug, Clone)]
struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

/// `WindowStack` is type for storing data (of [`i64`]) in stack
#[derive(Debug, Clone)]
pub struct WindowStack {
    head: Option<Box<Node>>,
}

impl Node {
    /// Create new [`Node`] with `value`
    pub fn new(value: i64) -> Self {
        Self { value, next: None }
    }

    /// Remove `value` from tree
    pub fn remove(mut self, value: i64) -> Option<Box<Self>> {
        if self.value == value {
            self.next
        } else {
            Some(Box::new(
                self.next
                    .take()
                    .map(|next| Self {
                        next: next.remove(value),
                        ..self
                    })
                    .unwrap_or(self),
            ))
        }
    }

    /// Move `value` to the head
    pub fn move_up(self, value: i64) -> Box<Self> {
        Box::new(Self {
            value: self.value,
            next: self.remove(value),
        })
    }

    /// Add `value` to the tail
    pub fn add(self, value: i64) -> Box<Self> {
        Box::new(Self {
            next: Some(
                self.next
                    .map(|next| next.add(value))
                    .unwrap_or(Box::new(Node::new(value))),
            ),
            ..self
        })
    }

    /// Try get value in `depth` of three
    pub fn get(&self, depth: usize) -> Option<i64> {
        if depth == 0 {
            Some(self.value)
        } else {
            self.next.as_ref().and_then(|next| next.get(depth - 1))
        }
    }

    /// Depth of three
    pub fn depth(&self) -> usize {
        self.next.as_ref().map(|next| next.depth()).unwrap_or(0) + 1
    }
}

impl WindowStack {
    /// Create new empty [`WindowStack`]
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Move window with `id` to the up of stack
    pub fn move_up(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.move_up(id))
            .or(Some(Box::new(Node::new(id))));
        id
    }

    /// Add window with `id` to the down of stack
    pub fn add(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.add(id))
            .or(Some(Box::new(Node::new(id))));
        id
    }

    /// Remove window with `id` from the stack
    pub fn remove(&mut self, id: i64) -> i64 {
        self.head = self.head.take().and_then(|head| head.remove(id));
        id
    }

    /// Get window `id` in `depth` of stack
    pub fn get(&self, depth: usize) -> Option<i64> {
        self.head.as_ref().and_then(|head| head.get(depth))
    }

    /// Get depth of stack
    pub fn depth(&self) -> usize {
        self.head.as_ref().map(|head| head.depth()).unwrap_or(0)
    }
}
