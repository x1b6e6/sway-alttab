use {
    async_stream::try_stream,
    evdev_rs_tokio::{enums::EventType, util::int_to_event_code, InputEvent, TimeVal},
    futures_core::Stream,
    nix::libc::input_event,
    std::{mem, path::Path, str},
    tokio::{
        fs::{self, File},
        io::{self, AsyncReadExt as _},
    },
};

/// Size of [`input_event`] from system
const INPUT_EVENT_SIZE: usize = mem::size_of::<input_event>();

/// Create asynchronous event stream from `file`
pub async fn new_stream(file: File) -> io::Result<impl Stream<Item = io::Result<InputEvent>>> {
    let mut file = Box::pin(file);
    Ok(try_stream! {
        loop {
            let mut buf = [0u8; INPUT_EVENT_SIZE];
            file.read_exact(&mut buf).await?;
            if let Some(ev) = input_event_from_buf(buf) {
                yield ev;
            }
        }
    })
}

/// Create [`InputEvent`] from byte array
fn input_event_from_buf(buf: [u8; INPUT_EVENT_SIZE]) -> Option<InputEvent> {
    let ev: input_event = unsafe { mem::transmute(buf) };
    if ev.type_ != EventType::EV_KEY as u16 {
        return None;
    }

    let event_code = int_to_event_code(ev.type_ as u32, ev.code as u32);

    Some(InputEvent {
        time: TimeVal {
            tv_sec: ev.time.tv_sec,
            tv_usec: ev.time.tv_usec,
        },
        event_code,
        value: ev.value,
    })
}

/// Try find keyboard device in `/sys/class/input/`
///
/// * look at each folder in the `/sys/class/input/`
/// * look at their key capabilities
/// * choose with large capabilities
/// * get path to event device at `/sys/class/input/<eventX>/uevent`
/// * return opened path at this event device
pub async fn try_find_keyboard() -> io::Result<Vec<String>> {
    let mut sys_class = fs::read_dir("/sys/class/input/").await?;
    let mut out = vec![];

    while let Some(dev) = sys_class.next_entry().await? {
        let dev_path = dev.path();
        if !dev_path.is_dir() {
            continue;
        }
        let uevent_path = dev_path.join("uevent");
        let mut buf = vec![];
        File::open(uevent_path).await?.read_to_end(&mut buf).await?;
        let uevent: String = str::from_utf8(&buf).unwrap().into();
        let lines = uevent.lines();
        for line in lines {
            let line: Vec<_> = line.split("=").collect();
            if line[0] == "DEVNAME" {
                let devname = Path::new("/dev/").join(line[1]);
                dbg!(&devname);
                if let Some(devname) = devname.to_str() {
                    out.push(devname.to_string());
                }
            }
        }
    }

    Ok(out)
}
