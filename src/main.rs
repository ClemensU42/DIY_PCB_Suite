pub mod io;
pub mod ui;

fn main() {
    dioxus_desktop::launch(ui::app::app);
}
