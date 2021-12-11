use crate::stack::Stack;

/// Hold stack of windows
///
/// The main function is preview of window
#[derive(Debug, Clone)]
pub struct StackHolder {
    window_stack: Stack,
    preview_depth: usize,
}

impl StackHolder {
    /// Create new [`StackHolder`]
    pub fn new() -> Self {
        Self {
            window_stack: Stack::new(),
            preview_depth: 0,
        }
    }

    /// Move window with `id` to up of stack
    pub fn move_up(&mut self, id: i64) {
        self.window_stack.move_up(id);
    }

    /// Add new window with `id`
    pub fn add(&mut self, id: i64) {
        self.window_stack.add(id);
    }

    /// Remove window with `id`
    pub fn remove(&mut self, id: i64) {
        self.window_stack.remove(id);
    }

    pub fn get(&self, depth: usize) -> Option<i64> {
        self.window_stack.get(depth)
    }

    pub fn depth(&self) -> usize {
        self.window_stack.depth()
    }

    /// Finish preview and move currently focused window to the up
    pub fn preview_finish(&mut self) {
        if let Some(id) = self.window_stack.get(self.preview_depth) {
            self.window_stack.move_up(id);
        }
        self.preview_depth = 0;
    }

    /// Select and focus next window (w/o moving windows in stack)
    pub fn preview_next(&mut self) -> Option<i64> {
        self.preview_depth += 1;
        self.window_stack.get(self.preview_depth).or_else(|| {
            self.preview_depth = 0;
            self.window_stack.get(0)
        })
    }

    /// Select and focus to previously window (w/o moving windows in stack)
    pub fn preview_prev(&mut self) -> Option<i64> {
        self.preview_depth
            .checked_sub(1)
            .and_then(|depth| {
                self.preview_depth = depth;
                self.window_stack.get(depth)
            })
            .or_else(|| {
                self.window_stack.depth().checked_sub(1).and_then(|depth| {
                    self.preview_depth = depth;
                    self.window_stack.get(depth)
                })
            })
    }
}

#[cfg(test)]
mod test {
    use super::StackHolder;

    #[test]
    fn preview_next() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        assert_eq!(s.preview_next(), Some(2));
        assert_eq!(s.preview_next(), Some(3));
        assert_eq!(s.preview_next(), Some(1));
        assert_eq!(s.preview_next(), Some(2));
        assert_eq!(s.preview_next(), Some(3));
        assert_eq!(s.preview_next(), Some(1));
    }

    #[test]
    fn preview_next_0() {
        let mut s = StackHolder::new();

        assert_eq!(s.preview_next(), None);
        assert_eq!(s.preview_next(), None);
        assert_eq!(s.preview_next(), None);
    }
    
    #[test]
    fn preview_next_1() {
        let mut s = StackHolder::new();

        s.add(1);

        assert_eq!(s.preview_next(), Some(1));
        assert_eq!(s.preview_next(), Some(1));
        assert_eq!(s.preview_next(), Some(1));
    }

    #[test]
    fn preview_prev() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        assert_eq!(s.preview_prev(), Some(3));
        assert_eq!(s.preview_prev(), Some(2));
        assert_eq!(s.preview_prev(), Some(1));
        assert_eq!(s.preview_prev(), Some(3));
        assert_eq!(s.preview_prev(), Some(2));
        assert_eq!(s.preview_prev(), Some(1));
        assert_eq!(s.preview_prev(), Some(3));
    }

    #[test]
    fn preview_prev_0() {
        let mut s = StackHolder::new();

        assert_eq!(s.preview_prev(), None);
        assert_eq!(s.preview_prev(), None);
        assert_eq!(s.preview_prev(), None);
    }
    
    #[test]
    fn preview_prev_1() {
        let mut s = StackHolder::new();

        s.add(1);

        assert_eq!(s.preview_prev(), Some(1));
        assert_eq!(s.preview_prev(), Some(1));
        assert_eq!(s.preview_prev(), Some(1));
    }

    #[test]
    fn preview_finish_0() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.preview_finish();

        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);

        s.preview_finish();

        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn preview_finish_1() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(2));
        assert_eq!(s.get(1), Some(1));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);

        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn preview_finish_2() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.preview_next();
        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(3));
        assert_eq!(s.get(1), Some(1));
        assert_eq!(s.get(2), Some(2));
        assert_eq!(s.get(3), None);

        s.preview_next();
        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(2));
        assert_eq!(s.get(1), Some(3));
        assert_eq!(s.get(2), Some(1));
        assert_eq!(s.get(3), None);
    }

    #[test]
    fn preview_finish_3() {
        let mut s = StackHolder::new();

        s.add(1);
        s.add(2);
        s.add(3);

        s.preview_next();
        s.preview_next();
        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);

        s.preview_next();
        s.preview_next();
        s.preview_next();
        s.preview_finish();

        assert_eq!(s.get(0), Some(1));
        assert_eq!(s.get(1), Some(2));
        assert_eq!(s.get(2), Some(3));
        assert_eq!(s.get(3), None);
    }
}
