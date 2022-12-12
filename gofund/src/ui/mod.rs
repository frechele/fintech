use cursive::views::Dialog;
use cursive::{menu, Cursive};
use cursive::event::Key;

use crate::Config;

pub fn create_ui(config: Config) {
    let mut siv = cursive::default();

    siv.set_window_title("Goindol Fund Client");

    build_menubar(&mut siv);

    siv.add_layer(Dialog::info(format!(
        "Server ip: {}\nServer port: {}",
        &config.ip, &config.port
    )));

    siv.run();
}

fn build_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree("File", menu::Tree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "Help",
            menu::Tree::new().leaf("About", |s| {
                s.add_layer(Dialog::info("Goindol Fund Client"));
            }),
        );

    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| { s.select_menubar(); });
}
