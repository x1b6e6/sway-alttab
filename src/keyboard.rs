use {
    async_stream::try_stream,
    evdev_rs_tokio::{util::int_to_event_code, InputEvent, TimeVal},
    futures_core::Stream,
    nix::libc::input_event,
    std::path::Path,
    tokio::{
        fs::{self, File},
        io::{self, AsyncReadExt},
    },
};

/// Size of [`input_event`] from system
const INPUT_EVENT_SIZE: usize = core::mem::size_of::<input_event>();

/// Create asynchronous keyboard event stream from `file`
pub async fn new_stream(file: File) -> io::Result<impl Stream<Item = io::Result<InputEvent>>> {
    let mut file = Box::pin(file);
    Ok(try_stream! {
        loop {
            let mut buf = [0u8; INPUT_EVENT_SIZE];
            let n = file.read(&mut buf).await?;
            if n != INPUT_EVENT_SIZE{
                return Err(io::ErrorKind::UnexpectedEof)?;
            }
            if let Some(ev) = input_event_from_buf(&buf) {
                yield ev;
            }
        }
    })
}

/// Create [`InputEvent`] from byte array
fn input_event_from_buf(buf: &[u8; INPUT_EVENT_SIZE]) -> Option<InputEvent> {
    let ev: input_event = unsafe { core::mem::transmute_copy(buf) };
    if ev.type_ != 1 {
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
/// * return opened [`File`] at this event device
pub async fn try_find_keyboard() -> io::Result<Option<fs::File>> {
    let mut sys_class = fs::read_dir("/sys/class/input/").await?;

    while let Some(dev) = sys_class.next_entry().await? {
        let dev_path = dev.path();
        if !dev_path.is_dir() {
            continue;
        }
        let key_cap_path = dev_path.join("device/capabilities/key");
        if !key_cap_path.is_file() {
            continue;
        }
        let mut buf = vec![];
        File::open(key_cap_path)
            .await?
            .read_to_end(&mut buf)
            .await?;
        let cap: String = std::str::from_utf8(&buf).unwrap().into();
        let cap = cap.replace("\n", "");
        let caps = cap.split(" ");
        if caps.last().unwrap() == "0" {
            continue;
        }
        let uevent_path = dev_path.join("uevent");
        buf = vec![];
        File::open(uevent_path).await?.read_to_end(&mut buf).await?;
        let uevent: String = std::str::from_utf8(&buf).unwrap().into();
        let lines = uevent.lines();
        for line in lines {
            let line: Vec<_> = line.split("=").collect();
            if line[0] == "DEVNAME" {
                let devname = Path::new("/dev/").join(line[1]);
                dbg!(&devname);
                let file = File::open(devname).await?;
                return Ok(Some(file));
            }
        }
    }

    Ok(None)
}
