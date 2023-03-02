use druid::{WindowDesc, AppLauncher};
use ui::ui_builder;

mod ui;
mod data;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("my app thingy")
        .window_size((400., 400.));
    AppLauncher::with_window(main_window)
        .launch(data::TodoState::default())
        .expect("Failed to start")
}
