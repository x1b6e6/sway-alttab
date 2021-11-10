use {
    evdev_rs_tokio::{enums::EV_KEY, InputEvent},
    stack_holder::StackHolder,
    swayipc_async as sway,
};

pub mod keyboard;
mod stack_holder;
mod window_stack;

#[derive(Debug)]
pub struct SwayAlttab {
    key_tab: EV_KEY,
    key_alt: EV_KEY,
    key_sft: EV_KEY,

    psd_alt: bool,
    psd_sft: bool,

    stack_holder: StackHolder,
}

impl SwayAlttab {
    pub async fn new(
        key_tab: EV_KEY,
        key_alt: EV_KEY,
        key_sft: EV_KEY,
    ) -> Result<Self, sway::Error> {
        let mut conn = sway::Connection::new().await?;
        let mut stack_holder = StackHolder::new();

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
        match tree.node_type {
            sway::NodeType::Con => vec![tree],
            _ => tree.nodes.iter().flat_map(SwayAlttab::nodes).collect(),
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
