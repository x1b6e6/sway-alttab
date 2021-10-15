use {
    super::window_stack,
    std::any::Any,
    std::sync::mpsc::{channel, Receiver, Sender},
    std::sync::{Arc, Mutex},
    std::thread,
    swayipc::Connection,
};

#[derive(Debug)]
pub enum Direction {
    Next,
    Prev,
}

#[derive(Debug)]
pub enum Action {
    MoveUp(i64),
    Add(i64),
    Remove(i64),
    Preview(Direction),
    PreviewEnd,
}

pub struct Service {
    worker: Option<(thread::JoinHandle<()>, Arc<Mutex<Sender<Action>>>)>,
}

impl Service {
    pub fn new() -> Self {
        Self { worker: None }
    }
    pub fn run_daemon(
        &mut self,
    ) -> Result<Arc<Mutex<Sender<Action>>>, Box<dyn Any + Send + 'static>> {
        if let None = self.worker {
            let (sender, receiver) = channel();
            let mtx = Mutex::from(sender);
            let sender = Arc::new(mtx);
            self.worker = Some((
                thread::spawn(move || SubWorker::new().run(receiver)),
                Arc::clone(&sender),
            ));
            Ok(sender)
        } else {
            Err(Box::<()>::from(()))
        }
    }

    pub fn join(&mut self) -> Result<(), Box<(dyn Any + Send + 'static)>> {
        if let Some((w, _)) = std::mem::replace(&mut self.worker, None) {
            w.join()
        } else {
            Err(Box::<()>::from(()))
        }
    }
}

struct SubWorker {
    window_stack: window_stack::WindowStack,
    preview_depth: usize,
    in_preview: bool,
}

impl SubWorker {
    pub fn new() -> Self {
        Self {
            window_stack: window_stack::WindowStack::new(),
            preview_depth: 0,
            in_preview: false,
        }
    }

    fn move_up(&mut self, id: i64) {
        if !self.in_preview {
            dbg!(self.window_stack.move_up(id));
        }
    }
    fn add(&mut self, id: i64) {
        dbg!(self.window_stack.add(id));
    }
    fn remove(&mut self, id: i64) {
        dbg!(self.window_stack.remove(id));
    }
    fn preview_end(&mut self) {
        if let Some(id) = self.window_stack.get(self.preview_depth) {
            self.window_stack.move_up(id);
        }
        self.preview_depth = 0;
        self.in_preview = false;
    }
    fn preview_next(&mut self) {
        self.in_preview = true;
        let mut depth = self.preview_depth;
        let mut sway = Connection::new().unwrap();

        depth += 1;

        let id = if let Some(id) = self.window_stack.get(depth) {
            id
        } else if depth >= self.window_stack.depth() {
            depth = 0;
            if let Some(id) = self.window_stack.get(depth) {
                id
            } else {
                dbg!("no windows");
                return;
            }
        } else {
            dbg!("unknown error");
            return;
        };

        let command = format!("[con_id={}] focus", id);
        dbg!(&command);
        sway.run_command(command).unwrap();
        self.preview_depth = depth;
    }
    fn preview_prev(&mut self) {
        self.in_preview = true;
        let mut depth = self.preview_depth as isize;
        let mut sway = Connection::new().unwrap();

        depth -= 1;

        let id = if let Some(id) = self.window_stack.get(depth as usize) {
            id
        } else if depth <= -1 {
            depth = self.window_stack.depth() as isize - 1;

            if let Some(id) = self.window_stack.get(depth as usize) {
                id
            } else {
                dbg!("no windows");
                return;
            }
        } else {
            dbg!("unknown error");
            return;
        };

        let command = format!("[con_id={}] focus", id);
        dbg!(&command);
        sway.run_command(command).unwrap();
        self.preview_depth = depth as usize;
    }

    pub fn run(&mut self, receiver: Receiver<Action>) -> ! {
        loop {
            let action = receiver.recv().unwrap();
            dbg!(&action);
            match action {
                Action::MoveUp(id) => self.move_up(id),
                Action::Add(id) => self.add(id),
                Action::Remove(id) => self.remove(id),
                Action::Preview(dir) => match dir {
                    Direction::Next => self.preview_next(),
                    Direction::Prev => self.preview_prev(),
                },
                Action::PreviewEnd => self.preview_end(),
            }
        }
    }
}
