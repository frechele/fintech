use cursive::views::Dialog;

fn main() {
    let mut siv = cursive::default();

    siv.set_window_title("Goindol Fund Client");

    siv.add_layer(
        Dialog::text("Unimplemented")
            .title("Information")
            .button("OK", |s| s.quit()),
    );

    siv.run();
}
