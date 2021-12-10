use {
    crate::window_stack::WindowStack,
    swayipc_async::{Connection, Error},
};

/// Hold stack of windows
///
/// The main function is preview of window
///
/// it's will show you window before
/// moving it to the top of stack
#[derive(Debug)]
pub struct StackHolder {
    window_stack: WindowStack,
    preview_depth: usize,
    ignore_move_up: Option<i64>,
}

impl StackHolder {
    /// Create new [`StackHolder`]
    pub fn new() -> Self {
        Self {
            window_stack: WindowStack::new(),
            preview_depth: 0,
            ignore_move_up: None,
        }
    }

    /// Move window with `id` to up of stack
    pub fn move_up(&mut self, id: i64) {
        if self.ignore_move_up != Some(id) {
            self.window_stack.move_up(id);
        } else {
            self.ignore_move_up.take();
        }
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
        self.ignore_move_up.take();
    }

    /// Select and focus next window (w/o moving windows in stack)
    pub async fn preview_next(&mut self) -> Result<(), Error> {
        let mut depth = self.preview_depth;
        let mut sway = Connection::new().await?;

        depth += 1;

        let id = if let Some(id) = self.window_stack.get(depth) {
            id
        } else if depth >= self.window_stack.depth() {
            depth = 0;
            if let Some(id) = self.window_stack.get(depth) {
                id
            } else if self.window_stack.depth() == 0 {
                return Ok(());
            } else {
                unreachable!()
            }
        } else {
            unreachable!();
        };

        self.ignore_move_up = Some(id);
        self.preview_depth = depth;

        let command = format!("[con_id={}] focus", id);
        sway.run_command(command).await?;

        Ok(())
    }

    /// Select and focus to previously window (w/o moving windows in stack)
    pub async fn preview_prev(&mut self) -> Result<(), Error> {
        let mut depth = self.preview_depth as isize;
        let mut sway = Connection::new().await?;

        depth -= 1;

        let id = if let Some(id) = self.window_stack.get(depth as usize) {
            id
        } else if depth == -1 {
            depth = self.window_stack.depth() as isize - 1;

            if let Some(id) = self.window_stack.get(depth as usize) {
                id
            } else if self.window_stack.depth() == 0 {
                return Ok(());
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        };

        self.ignore_move_up = Some(id);
        self.preview_depth = depth as usize;

        let command = format!("[con_id={}] focus", id);
        sway.run_command(command).await?;

        Ok(())
    }
}
