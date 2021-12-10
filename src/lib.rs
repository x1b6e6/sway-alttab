use {
    evdev_rs_tokio::{
        enums::{EventCode, EV_KEY},
        InputEvent,
    },
    stack_holder::StackHolder,
    swayipc_async::{
        Connection, Error, Event, EventStream, EventType, Node, NodeLayout, NodeType, WindowChange,
    },
};

pub mod keyboard;
pub mod stack_holder;
pub mod window_stack;

/// `SwayAlttab` is type with main logic of application
#[derive(Debug)]
pub struct SwayAlttab {
    /// key with Tab behavior
    key_tab: EV_KEY,
    /// key with Alt behavior
    key_alt: EV_KEY,
    /// key with Shift behavior
    key_sft: EV_KEY,

    /// key with Alt behavior is pressed
    psd_alt: bool,
    /// key with Shift behavior is pressed
    psd_sft: bool,

    /// windows stack in [`StackHolder`]
    stack_holder: StackHolder,
}

impl SwayAlttab {
    /// Create [`SwayAlttab`] object with params
    /// `key_tab` is key with Tab behavior
    /// `key_alt` is key with Alt behavior
    /// `key_sft` is key with Shift behavior
    pub async fn new(key_tab: EV_KEY, key_alt: EV_KEY, key_sft: EV_KEY) -> Result<Self, Error> {
        let mut conn = Connection::new().await?;
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

    /// Try create stream of [`EventStream`]
    pub async fn sway_events() -> Result<EventStream, Error> {
        Connection::new()
            .await?
            .subscribe(&[EventType::Window])
            .await
    }

    /// Get vector of windows as [`Node`]
    fn nodes(tree: &Node) -> Vec<&Node> {
        match tree.node_type {
            NodeType::Con if tree.layout == NodeLayout::None => vec![tree],
            NodeType::FloatingCon => vec![tree],
            _ => tree
                .nodes
                .iter()
                .chain(tree.floating_nodes.iter())
                .flat_map(SwayAlttab::nodes)
                .collect(),
        }
    }

    /// Process keyboard event [`InputEvent`]
    pub async fn process_keyboard_event(&mut self, event: InputEvent) -> Result<(), Error> {
        if let EventCode::EV_KEY(key) = event.event_code {
            if key == self.key_alt {
                self.psd_alt = event.value > 0;
                if !self.psd_alt {
                    self.stack_holder.preview_finish();
                }
            } else if key == self.key_sft {
                self.psd_sft = event.value > 0;
            } else if key == self.key_tab && self.psd_alt && event.value == 1 {
                if !self.psd_sft {
                    self.stack_holder.preview_next().await?;
                } else {
                    self.stack_holder.preview_prev().await?;
                }
            }
        }

        Ok(())
    }

    /// Process sway event [`Event`]
    pub fn process_sway_event(&mut self, event: Event) {
        match event {
            Event::Window(window) => {
                let id = window.container.id;
                match window.change {
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
