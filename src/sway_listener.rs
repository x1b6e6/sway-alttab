use {
    super::stack_holder::{Action, Direction},
    std::any::Any,
    std::sync::mpsc::Sender,
    std::sync::{Arc, Mutex},
    std::thread,
    swayipc::{Connection, Event, EventType, Node, NodeType, WindowChange},
};

pub struct Listener {
    worker: Option<thread::JoinHandle<()>>,
}

impl Listener {
    pub fn new() -> Self {
        Self { worker: None }
    }

    pub fn run_daemon(
        &mut self,
        sender: &Arc<Mutex<Sender<Action>>>,
    ) -> Result<(), Box<dyn Any + Send + 'static>> {
        if let None = self.worker {
            let sender = Arc::clone(&sender);
            self.worker = Some(thread::spawn(move || undaemon(sender)));
            Ok(())
        } else {
            Err(Box::<()>::from(()))
        }
    }
    pub fn join(&mut self) -> Result<(), Box<(dyn Any + Send + 'static)>> {
        if let Some(w) = std::mem::replace(&mut self.worker, None) {
            w.join()
        } else {
            Err(Box::<()>::from(()))
        }
    }
}

fn undaemon(sender: Arc<Mutex<Sender<Action>>>) -> ! {
    let mut conn = Connection::new().unwrap();
    {
        let tree = conn.get_tree().unwrap();
        let s = sender.lock().unwrap();

        let nodes = nodes(&tree);
        nodes
            .iter()
            .for_each(|x| s.send(Action::Add(x.id)).unwrap());
        if let Some(node) = nodes.iter().find(|x| x.focused) {
            s.send(Action::MoveUp(node.id)).unwrap();
        }
    }
    let mut events = conn
        .subscribe(&[EventType::Window, EventType::Binding])
        .unwrap();
    loop {
        if let Some(ev) = events.next() {
            if let Ok(ev) = ev {
                match ev {
                    Event::Window(w) => {
                        let id = w.container.id;
                        if let Ok(sender) = sender.lock() {
                            match w.change {
                                WindowChange::Focus => sender.send(Action::MoveUp(id)).unwrap(),
                                WindowChange::New => sender.send(Action::Add(id)).unwrap(),
                                WindowChange::Close => sender.send(Action::Remove(id)).unwrap(),
                                _ => {}
                            }
                        }
                    }
                    Event::Binding(b) => {
                        let cmd = dbg!(b.binding.command);
                        if cmd == "sway-alttab next" {
                            if let Ok(s) = sender.lock() {
                                s.send(Action::Preview(Direction::Next)).unwrap();
                            }
                        } else if cmd == "sway-alttab prev" {
                            if let Ok(s) = sender.lock() {
                                s.send(Action::Preview(Direction::Prev)).unwrap();
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

fn nodes(tree: &Node) -> Vec<&Node> {
    if let NodeType::Con = tree.node_type {
        vec![tree]
    } else {
        tree.nodes.iter().fold(vec![], |mut all, x| {
            let mut x = nodes(x);
            all.append(&mut x);
            all
        })
    }
}
