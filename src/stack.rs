/// `Node` is internal type for storing data (of [`i64`]) in [`Stack`]
#[derive(Debug, Clone)]
struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

/// `Stack` is type for storing data (of [`i64`]) in stack
#[derive(Debug, Clone)]
pub struct Stack {
    head: Option<Box<Node>>,
}

impl Node {
    /// Create new [`Node`] with `value`
    pub fn new(value: i64) -> Self {
        Self { value, next: None }
    }

    /// Remove `value` from stack
    pub fn remove(self, value: i64) -> Option<Box<Self>> {
        if self.value == value {
            self.next
        } else {
            Some(Box::new(Self {
                value: self.value,
                next: self.next.and_then(|next| next.remove(value)),
            }))
        }
    }

    /// Move `value` to the head
    pub fn move_up(self, value: i64) -> Box<Self> {
        Box::new(Self {
            value,
            next: self.remove(value),
        })
    }

    /// Add `value` to the tail
    pub fn add(mut self, value: i64) -> Box<Self> {
        if self.value != value {
            self.next = self
                .next
                .map(|next| next.add(value))
                .or_else(|| Some(Box::new(Node::new(value))));
        }
        Box::new(self)
    }

    /// Try get value in `depth` of stack
    pub fn get(&self, depth: usize) -> Option<i64> {
        if depth == 0 {
            Some(self.value)
        } else {
            self.next.as_ref().and_then(|next| next.get(depth - 1))
        }
    }

    /// Depth of stack
    pub fn depth(&self) -> usize {
        self.next.as_ref().map(|next| next.depth()).unwrap_or(0) + 1
    }
}

impl Stack {
    /// Create new empty [`Stack`]
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Move window with `id` to the up of stack
    pub fn move_up(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.move_up(id))
            .or_else(|| Some(Box::new(Node::new(id))));
        id
    }

    /// Add window with `id` to the down of stack
    pub fn add(&mut self, id: i64) -> i64 {
        self.head = self
            .head
            .take()
            .map(|head| head.add(id))
            .or_else(|| Some(Box::new(Node::new(id))));
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

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn add() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        assert_eq!(s.depth(), 3);
        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn remove_mid() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.remove(2);

        assert_eq!(s.depth(), 2);
        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(3));
        assert_eq!(s.get(2), None);
    }

    #[test]
    fn remove_top() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.remove(1);

        assert_eq!(s.depth(), 2);
        assert_eq!(s.get(0), Some(2));
        assert_eq!(s.get(1), Some(3));
        assert_eq!(s.get(2), None);
    }

    #[test]
    fn remove_tail() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.remove(3);

        assert_eq!(s.depth(), 2);
        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), None);
    }

    #[test]
    fn remove_not_found() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.remove(4);

        assert_eq!(s.depth(), 3);
        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn move_up_mid() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.move_up(2);

        assert_eq!(s.depth(), 3);
        assert_eq!(s.get(0), Some(2));
        assert_eq!(s.get(1), Some(1));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn move_up_tail() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.move_up(3);

        assert_eq!(s.depth(), 3);
        assert_eq!(s.get(0), Some(3));
        assert_eq!(s.get(1), Some(1));
        assert_eq!(s.get(2), Some(2));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn move_up_head() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.move_up(1);

        assert_eq!(s.depth(), 3);
        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn move_up_not_found() {
        let mut s = Stack::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.move_up(4);

        assert_eq!(s.depth(), 4);
        assert_eq!(s.get(0), Some(4));
        assert_eq!(s.get(1), Some(1));
        assert_eq!(s.get(2), Some(2));
        assert_eq!(s.get(3), Some(3));
        assert_eq!(s.get(4), None);
    }
}
