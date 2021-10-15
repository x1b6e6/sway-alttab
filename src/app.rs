use clap::{App, AppSettings, Arg};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn build_app() -> App<'static, 'static> {
    let clap_color_setting = if std::env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .about(PKG_DESCRIPTION)
        .setting(clap_color_setting)
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .value_name("DEVICE")
                .help("keyboard device")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("alt")
                .long("key-alt")
                .value_name("KEY_ALT")
                .help("key for alt-tab with alt behavior")
                .takes_value(true)
                .default_value("KEY_LEFTALT"),
        )
        .arg(
            Arg::with_name("shift")
                .long("key-shift")
                .value_name("KEY_SHIFT")
                .help("key for inverse direction of alt-tab")
                .takes_value(true)
                .default_value("KEY_LEFTSHIFT"),
        )
        .arg(
            Arg::with_name("tab")
                .long("key-tab")
                .value_name("KEY_TAB")
                .help("key for alt-tab with tab behavior")
                .takes_value(true)
                .default_value("KEY_TAB"),
        )
}
