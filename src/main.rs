///! Program for switching between windows on swaywm
use {
    evdev_rs_tokio::{
        enums::{EventCode, EV_KEY},
        InputEvent,
    },
    futures_util::{pin_mut, StreamExt as _},
    nix::unistd::{getgroups, getuid, setgid, setuid, Gid, Uid},
    stack_holder::StackHolder,
    std::{
        os::unix::fs::{MetadataExt as _, PermissionsExt as _},
        path::Path,
        str::FromStr as _,
    },
    swayipc_async::{
        Connection, Error, Event, EventStream, EventType, Fallible, Node, NodeLayout, NodeType,
        WindowChange,
    },
    tokio::{fs, io, select},
};

mod app;
mod keyboard;
mod stack_holder;
mod window_stack;

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

    /// ignore `focus` event with this window id (for preview mode)
    ignore_move_up: Option<i64>,
}

impl SwayAlttab {
    /// Create [`SwayAlttab`] object with params
    /// `key_tab` is key with Tab behavior
    /// `key_alt` is key with Alt behavior
    /// `key_sft` is key with Shift behavior
    pub async fn new(key_tab: EV_KEY, key_alt: EV_KEY, key_sft: EV_KEY) -> Result<Self, Error> {
        let mut swayalttab = Self {
            key_tab,
            key_alt,
            key_sft,

            psd_alt: false,
            psd_sft: false,

            stack_holder: StackHolder::new(),
            ignore_move_up: None,
        };

        swayalttab.refresh_nodes().await?;

        Ok(swayalttab)
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
            NodeType::Workspace if tree.name == Some("__i3_scratch".to_string()) => vec![],
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

    async fn refresh_nodes(&mut self) -> Result<(), Error> {
        let mut sway = Connection::new().await?;
        let root = sway.get_tree().await?;
        let nodes = Self::nodes(&root);
        let size = self.stack_holder.depth();
        nodes.iter().for_each(|node| self.stack_holder.add(node.id));
        if size != self.stack_holder.depth() {
            let size = self.stack_holder.depth();
            if let Some(id) = self.stack_holder.get(size - 1) {
                self.stack_holder.move_up(id);
            }
        }
        if let Some(node) = nodes.iter().find(|node| node.focused) {
            self.stack_holder.move_up(node.id);
        }
        Ok(())
    }

    /// Focus window in preview mode
    async fn preview(&mut self, id: i64) -> Result<(), Error> {
        let mut sway = Connection::new().await?;
        let cmd = format!("[con_id={}] focus", id);
        self.ignore_move_up = Some(id);
        let result = sway.run_command(cmd).await;
        result.map(|_| ()).map_err(|err| {
            self.ignore_move_up = None;
            err
        })
    }

    /// Process keyboard event [`InputEvent`]
    pub async fn process_keyboard_event(&mut self, event: InputEvent) -> Result<(), Error> {
        if let EventCode::EV_KEY(key) = event.event_code {
            if key == self.key_alt {
                self.psd_alt = event.value > 0;
                if !self.psd_alt {
                    self.stack_holder.preview_finish();
                    self.ignore_move_up = None;
                }
            } else if key == self.key_sft {
                self.psd_sft = event.value > 0;
            } else if key == self.key_tab && self.psd_alt && event.value == 1 {
                let id = if !self.psd_sft {
                    self.stack_holder.preview_next()
                } else {
                    self.stack_holder.preview_prev()
                };
                if let Some(id) = id {
                    self.preview(id).await?;
                }
            }
        }

        Ok(())
    }

    /// Process sway event [`Event`]
    pub async fn process_sway_event(&mut self, event: Event) -> Result<(), Error> {
        if let Event::Window(window) = event {
            let id = window.container.id;
            match window.change {
                WindowChange::New => self.stack_holder.add(id),
                WindowChange::Close => self.stack_holder.remove(id),
                WindowChange::Focus => {
                    if self.ignore_move_up != Some(id) {
                        self.stack_holder.move_up(id)
                    } else {
                        self.ignore_move_up = None;
                    }
                }
                WindowChange::Move => {
                    self.stack_holder.remove(id);
                    self.refresh_nodes().await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let args = app::build_app().get_matches_from(std::env::args_os());

    let device = args.value_of("device");
    let key_alt = args.value_of("alt").unwrap();
    let key_sft = args.value_of("shift").unwrap();
    let key_tab = args.value_of("tab").unwrap();

    let key_error = |key| format!("incorrect key {}", key);

    let key_alt = EV_KEY::from_str(key_alt).expect(&key_error(key_alt));
    let key_tab = EV_KEY::from_str(key_tab).expect(&key_error(key_tab));
    let key_sft = EV_KEY::from_str(key_sft).expect(&key_error(key_sft));

    let filename = if let Some(device) = device {
        device.to_string()
    } else {
        keyboard::try_find_keyboard()
            .await?
            .expect("can't found keyboard")
    };
    let filename = Path::new(&filename);

    let file = try_open_file(filename)
        .await
        .expect("device is not a keyboard or permission denied");

    let kb = keyboard::new_stream(file).await.unwrap();
    let swayalttab = SwayAlttab::new(key_tab, key_alt, key_sft).await.unwrap();
    let sway = SwayAlttab::sway_events().await.unwrap();

    pin_mut!(kb);
    pin_mut!(sway);
    pin_mut!(swayalttab);

    loop {
        select! {
            ev = kb.next() => {
                let ev = ev.expect("keyboard stream error")?;
                swayalttab.process_keyboard_event(ev).await?;
            }
            ev = sway.next() => {
                let ev = ev.expect("sway events stream error")?;
                swayalttab.process_sway_event(ev).await?;
            }
        };
    }
}

/// try open the file
/// if process doesn't have permissions then try get permissions
async fn try_open_file(filepath: &Path) -> io::Result<fs::File> {
    let meta = fs::metadata(filepath).await?;
    let uid = Uid::from_raw(meta.uid());
    let gid = Gid::from_raw(meta.gid());
    let mode = meta.permissions().mode();

    let have_read_permissions = ((mode & 0o004) == 0o004)
        || ((mode & 0o040) == 0o040 && getgroups().unwrap().contains(&gid))
        || ((mode & 0o400) == 0o400 && uid == getuid());

    if !have_read_permissions {
        let exe_meta = fs::metadata("/proc/self/exe").await?;
        let exe_mode = exe_meta.permissions().mode();
        if (exe_mode & 0o2000) == 0o2000 && gid == Gid::from_raw(exe_meta.gid()) {
            setgid(gid).ok();
        } else if (exe_mode & 0o4000) == 0o4000 && uid == Uid::from_raw(exe_meta.uid()) {
            setuid(uid).ok();
        }
    }

    fs::File::open(filepath).await
}
