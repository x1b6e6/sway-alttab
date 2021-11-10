use {super::window_stack, sway::Connection, swayipc_async as sway};
pub struct StackHolder {
    window_stack: window_stack::WindowStack,
    preview_depth: usize,
    in_preview: bool,
}

impl StackHolder {
    pub fn new() -> Self {
        Self {
            window_stack: window_stack::WindowStack::new(),
            preview_depth: 0,
            in_preview: false,
        }
    }

    pub fn move_up(&mut self, id: i64) {
        if !self.in_preview {
            self.window_stack.move_up(id);
        }
    }

    pub fn add(&mut self, id: i64) {
        self.window_stack.add(id);
    }

    pub fn remove(&mut self, id: i64) {
        self.window_stack.remove(id);
    }

    pub fn preview_end(&mut self) {
        if let Some(id) = self.window_stack.get(self.preview_depth) {
            self.window_stack.move_up(id);
        }
        self.preview_depth = 0;
        self.in_preview = false;
    }

    pub async fn preview_next(&mut self) -> Result<(), sway::Error> {
        self.in_preview = true;
        let mut depth = self.preview_depth;
        let mut sway = Connection::new().await?;

        depth += 1;

        let id = if let Some(id) = self.window_stack.get(depth) {
            id
        } else if depth >= self.window_stack.depth() {
            depth = 0;
            if let Some(id) = self.window_stack.get(depth) {
                id
            } else {
                return Ok(());
            }
        } else {
            unreachable!();
        };

        let command = format!("[con_id={}] focus", id);
        sway.run_command(command).await?;
        self.preview_depth = depth;

        Ok(())
    }

    pub async fn preview_prev(&mut self) -> Result<(), sway::Error> {
        self.in_preview = true;
        let mut depth = self.preview_depth as isize;
        let mut sway = Connection::new().await?;

        depth -= 1;

        let id = if let Some(id) = self.window_stack.get(depth as usize) {
            id
        } else if depth <= -1 {
            depth = self.window_stack.depth() as isize - 1;

            if let Some(id) = self.window_stack.get(depth as usize) {
                id
            } else {
                return Ok(());
            }
        } else {
            unreachable!();
        };

        let command = format!("[con_id={}] focus", id);
        sway.run_command(command).await?;
        self.preview_depth = depth as usize;

        Ok(())
    }
}
