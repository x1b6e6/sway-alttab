use {
    async_stream::try_stream,
    evdev_rs_tokio::{util::int_to_event_code, InputEvent, TimeVal},
    futures_core::Stream,
    nix::libc::input_event,
    std::path::Path,
    tokio::{
        fs::File,
        io::{self, AsyncReadExt},
    },
};

const INPUT_EVENT_SIZE: usize = core::mem::size_of::<input_event>();

pub async fn new_stream(
    dev: impl AsRef<Path>,
) -> io::Result<impl Stream<Item = io::Result<InputEvent>>> {
    let mut file = Box::pin(File::open(dev).await?);
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
