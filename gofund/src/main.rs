use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("고인돌 펀드~")
            .title("Information")
            .button("OK", |s| s.quit()),
    );

    siv.run();
}
