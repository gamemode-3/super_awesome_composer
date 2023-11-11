mod backend;
mod data;
mod music_canvas;

use druid::widget::{Button, CrossAxisAlignment, Flex, FlexParams};
use druid::{AppLauncher, Widget, WindowDesc};

use data::State;
use druid_shell::WindowState;
use music_canvas::music_canvas::MusicCanvas;

fn build_ui() -> impl Widget<State> {
    Flex::column()
        .with_child(top_bar())
        .with_flex_child(
            middle_canvas(),
            FlexParams::new(1.0, CrossAxisAlignment::End),
        )
        .with_child(bottom_bar())
}

fn top_bar() -> impl Widget<State> {
    Button::new("top bar button")
}

fn middle_canvas() -> impl Widget<State> {
    MusicCanvas::new()
}

fn bottom_bar() -> impl Widget<State> {
    Button::new("bottom bar button")
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App")
        .set_window_state(WindowState::Maximized);
    let initial_data = State::new();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}

#[test]
fn it_works() {
    // use backend::music::*;
    // let p = Project::default();
    // println!("{}", p);
    use backend::pitch::Pitch;

    println!("{}", Pitch::from_string("A4").unwrap().frequency());
    println!("{}", Pitch::from_string("A4").unwrap());
    println!("{}", Pitch::from_frequency(435.));
    println!("{}", Pitch::from_frequency(445.));
}
