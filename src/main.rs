use {
    evdev_rs_tokio::{enums::EV_KEY, InputEvent},
    futures_util::{pin_mut, StreamExt},
    nix::unistd::{setuid, Uid},
    std::str::FromStr,
    swayipc_async as sway,
    tokio::select,
};

mod app;
mod keyboard;
mod stack_holder;
mod window_stack;

#[tokio::main]
async fn main() {
    let args = app::build_app().get_matches_from(std::env::args_os());

    let device = args.value_of("device").unwrap();
    let key_alt = args.value_of("alt").unwrap();
    let key_sft = args.value_of("shift").unwrap();
    let key_tab = args.value_of("tab").unwrap();

    let key_error = |key| format!("incorrect key {}", key);

    let key_alt = EV_KEY::from_str(key_alt).expect(&key_error(key_alt));
    let key_tab = EV_KEY::from_str(key_tab).expect(&key_error(key_tab));
    let key_sft = EV_KEY::from_str(key_sft).expect(&key_error(key_sft));

    setuid(Uid::from_raw(0)).expect("error in setuid");

    let kb = keyboard::new_stream(String::from(device)).await.unwrap();
    let sway = swayipc_async::Connection::new()
        .await
        .unwrap()
        .subscribe(&[sway::EventType::Window])
        .await
        .unwrap();

    pin_mut!(kb);
    pin_mut!(sway);

    let swayalttab = SwayAlttab::new(key_tab, key_alt, key_sft).await.unwrap();
    pin_mut!(swayalttab);

    loop {
        select! {
            ev = kb.next() => {
                if let Some(ev) = ev {
                    let ev = ev.expect("keyboard stream error");
                    swayalttab.kb_ev(ev).await.expect("error while process keyboard event");
                } else {
                    break;
                }
            }
            ev = sway.next() => {
                if let Some(ev) = ev {
                    let ev = ev.expect("sway events stream error");
                    swayalttab.sway_ev(ev);
                } else {
                    break;
                }
            }
        };
    }
}

struct SwayAlttab {
    key_tab: EV_KEY,
    key_alt: EV_KEY,
    key_sft: EV_KEY,

    psd_alt: bool,
    psd_sft: bool,

    stack_holder: stack_holder::StackHolder,
}

impl SwayAlttab {
    pub async fn new(
        key_tab: EV_KEY,
        key_alt: EV_KEY,
        key_sft: EV_KEY,
    ) -> Result<Self, sway::Error> {
        let mut conn = sway::Connection::new().await?;
        let mut stack_holder = stack_holder::StackHolder::new();

        let tree = conn.get_tree().await?;

        let nodes = SwayAlttab::nodes(&tree);
        nodes.iter().for_each(|node| stack_holder.add(node.id));
        if let Some(node) = nodes.iter().find(|node| node.focused) {
            stack_holder.move_up(node.id);
        }

        Ok(Self {
            key_tab,
            key_alt,
            key_sft,

            psd_alt: false,
            psd_sft: false,

            stack_holder,
        })
    }

    fn nodes(tree: &sway::Node) -> Vec<&sway::Node> {
        if let sway::NodeType::Con = tree.node_type {
            vec![tree]
        } else {
            tree.nodes.iter().fold(vec![], |mut all, x| {
                let mut x = SwayAlttab::nodes(x);
                all.append(&mut x);
                all
            })
        }
    }

    pub async fn kb_ev(&mut self, ev: InputEvent) -> Result<(), sway::Error> {
        use evdev_rs_tokio::enums::EventCode;

        if let EventCode::EV_KEY(key) = ev.event_code {
            if key == self.key_alt {
                self.psd_alt = ev.value > 0;
                if !self.psd_alt {
                    self.stack_holder.preview_end();
                }
            } else if key == self.key_sft {
                self.psd_sft = ev.value > 0;
            } else if key == self.key_tab && self.psd_alt && ev.value == 1 {
                if !self.psd_sft {
                    self.stack_holder.preview_next().await?;
                } else {
                    self.stack_holder.preview_prev().await?;
                }
            }
        }

        Ok(())
    }

    pub fn sway_ev(&mut self, ev: sway::Event) {
        use swayipc_async::{Event, WindowChange};

        match ev {
            Event::Window(w) => {
                let id = w.container.id;
                match w.change {
                    WindowChange::Focus => self.stack_holder.move_up(id),
                    WindowChange::New => self.stack_holder.add(id),
                    WindowChange::Close => self.stack_holder.remove(id),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
