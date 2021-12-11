///! Program for switching between windows on swaywm
use {
    evdev_rs_tokio::enums::EV_KEY,
    futures_util::{pin_mut, StreamExt as _},
    nix::unistd::{getgroups, getuid, setgid, setuid, Gid, Uid},
    std::{
        os::unix::fs::{MetadataExt as _, PermissionsExt as _},
        path::Path,
        str::FromStr as _,
    },
    sway_alttab::{keyboard, SwayAlttab},
    swayipc_async::Fallible,
    tokio::{fs, io, select},
};

mod app;

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
                let ev = ev.expect("keyboard stream error").unwrap();
                swayalttab.process_keyboard_event(ev).await.unwrap();
            }
            ev = sway.next() => {
                let ev = ev.expect("sway events stream error").unwrap();
                swayalttab.process_sway_event(ev);
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
