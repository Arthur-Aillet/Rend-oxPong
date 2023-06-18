use rend_ox::app::{app, App};
use rend_ox::nannou_egui::egui::CtxRef;

pub struct Snake {

}

impl Snake {
    pub fn new() -> Snake {
        Snake{}
    }
}

pub fn snake_update<Snake>(_: &rend_ox::nannou::App, _: &mut App<Snake>, _: rend_ox::nannou::event::Update, _: CtxRef) {
}

fn snake_app(nannou_app: &rend_ox::nannou::App) -> App<Snake> {
    app(nannou_app, Snake::new(), snake_update)
}

fn main() {
    rend_ox::app::launch_rendox_app(snake_app);
}