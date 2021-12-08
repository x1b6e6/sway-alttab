///! Program for switching between windows on swaywm
use {
    evdev_rs_tokio::enums::EV_KEY,
    futures_util::{pin_mut, StreamExt as _},
    nix::unistd::{setuid, Uid},
    std::str::FromStr as _,
    sway_alttab::{keyboard, SwayAlttab},
    swayipc_async::Fallible,
    tokio::{fs, select},
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

    setuid(Uid::from_raw(0)).unwrap();

    let file = if let Some(device) = device {
        fs::File::open(device)
            .await
            .ok()
            .expect("device is not a keyboard")
    } else {
        keyboard::try_find_keyboard()
            .await?
            .expect("can't found keyboard")
    };

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
                swayalttab.kb_ev(ev).await.unwrap();
            }
            ev = sway.next() => {
                let ev = ev.expect("sway events stream error").unwrap();
                swayalttab.sway_ev(ev);
            }
        };
    }
}
