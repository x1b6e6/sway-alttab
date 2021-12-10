use crate::window_stack::WindowStack;

/// Hold stack of windows
///
/// The main function is preview of window
#[derive(Debug)]
pub struct StackHolder {
    window_stack: WindowStack,
    preview_depth: usize,
}

impl StackHolder {
    /// Create new [`StackHolder`]
    pub fn new() -> Self {
        Self {
            window_stack: WindowStack::new(),
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
