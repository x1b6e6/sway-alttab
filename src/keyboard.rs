use {
    super::stack_holder::{Action, Direction},
    evdev_rs_tokio::{
        enums::{EventCode, EV_KEY},
        InputEvent, ReadFlag, ReadStatus, UninitDevice,
    },
    std::sync::{mpsc::Sender, Arc, Mutex},
    tokio::fs::File,
};

pub struct Keyboard {
    meta_pressed: bool,
    reverse_pressed: bool,
    key_alt: EV_KEY,
    key_tab: EV_KEY,
    key_shift: EV_KEY,
    sender: Arc<Mutex<Sender<Action>>>,
}

impl Keyboard {
    pub fn new(
        key_alt: EV_KEY,
        key_tab: EV_KEY,
        key_shift: EV_KEY,
        sender: &Arc<Mutex<Sender<Action>>>,
    ) -> Self {
        Self {
            meta_pressed: false,
            reverse_pressed: false,
            key_alt,
            key_tab,
            key_shift,
            sender: sender.clone(),
        }
    }

    pub async fn wait(&mut self, filename: String) {
        let f = File::open(filename);
        let u_d = UninitDevice::new().unwrap();
        let d = u_d.set_file(f.await.unwrap()).unwrap();

        loop {
            let ev = d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING);
            if let Ok(ev) = ev {
                if let ReadStatus::Success = ev.0 {
                    self.process_event(ev.1).await;
                } else {
                    dbg!("ignore sync event");
                }
            }
        }
    }

    async fn process_event(&mut self, event: InputEvent) {
        if let EventCode::EV_KEY(key) = event.event_code {
            if self.key_alt == key {
                self.meta_pressed = event.value != 0;
                if event.value == 0 {
                    // end
                    self.sender
                        .lock()
                        .unwrap()
                        .send(Action::PreviewEnd)
                        .unwrap();
                }
            } else if self.key_shift == key {
                self.reverse_pressed = event.value != 0;
            } else if self.key_tab == key {
                if event.value == 1 && self.meta_pressed {
                    if !self.reverse_pressed {
                        // normal
                        self.sender
                            .lock()
                            .unwrap()
                            .send(Action::Preview(Direction::Next))
                            .unwrap();
                    } else {
                        // reverse
                        self.sender
                            .lock()
                            .unwrap()
                            .send(Action::Preview(Direction::Prev))
                            .unwrap();
                    }
                }
            }
        }
    }
}
