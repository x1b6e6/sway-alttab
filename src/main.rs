use {
    evdev_rs_tokio::enums::EV_KEY,
    nix::unistd::{setuid, Uid},
    std::str::FromStr,
};

mod app;
mod keyboard;
mod stack_holder;
mod sway_listener;
mod window_stack;

#[tokio::main]
async fn main() {
    let args = app::build_app().get_matches_from(std::env::args_os());

    let device = args.value_of("device").unwrap();
    let key_alt = args.value_of("alt").unwrap();
    let key_shift = args.value_of("shift").unwrap();
    let key_tab = args.value_of("tab").unwrap();

    let key_error = |key| format!("incorrect key {}", key);

    let mut stack_holder = stack_holder::Service::new();
    let sender = stack_holder.run_daemon().unwrap();

    let mut sway_listener = sway_listener::Listener::new();
    sway_listener.run_daemon(&sender).unwrap();

    setuid(Uid::from_raw(0)).expect("error in setuid");

    let mut kb = keyboard::Keyboard::new(
        EV_KEY::from_str(key_alt).expect(&key_error(key_alt)),
        EV_KEY::from_str(key_tab).expect(&key_error(key_tab)),
        EV_KEY::from_str(key_shift).expect(&key_error(key_shift)),
        &sender,
    );
    kb.wait(device.into()).await;

    stack_holder.join().unwrap();
    sway_listener.join().unwrap();
}
