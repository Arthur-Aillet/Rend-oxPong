use rend_ox::app::{app, App};
use rend_ox::mesh::MeshDescriptor;
use rend_ox::nannou::event::Key;
use rend_ox::nannou_egui::egui::CtxRef;
use rend_ox::Vec3;

pub struct Snake {
    pub ball: Option<MeshDescriptor>,
    pub rack: Option<MeshDescriptor>,
    pub fst_height: f32,
    pub snd_height: f32,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            ball: None,
            rack: None,
            fst_height: 0.0,
            snd_height: 0.0,
        }
    }
}

fn bound_val(val: f32, min: f32, max: f32) -> f32 {
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }
    val
}

pub fn pong_update(nannou: &rend_ox::nannou::App, app: &mut App<Snake>, update: rend_ox::nannou::event::Update, _: CtxRef) {
    rend_ox::camera_controller::default_camera(nannou, app, &update);
    if nannou.keys.down.contains(&Key::T) {
        app.user.fst_height += 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    if nannou.keys.down.contains(&Key::G) {
        app.user.fst_height -= 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    app.user.fst_height = bound_val(app.user.fst_height, -17., 17.);

    if nannou.keys.down.contains(&Key::O) {
        app.user.snd_height += 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    if nannou.keys.down.contains(&Key::L) {
        app.user.snd_height -= 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    app.user.snd_height = bound_val(app.user.snd_height, -17., 17.);

    if let Some(mesh) = &app.user.ball {
        app.draw_at(mesh, Vec3::new(0.2, 1., 12.), Vec3::new(0., -30., 0.), Vec3::new(0., 0., 0.), Vec3::new(2., 2., 2.));
    }
    if let Some(mesh) = &app.user.rack {
        app.draw_at(mesh, Vec3::new(1., 1., 12.), Vec3::new(30., -30., app.user.fst_height), Vec3::new(0., std::f32::consts::PI/2., std::f32::consts::PI/2.), Vec3::new(2., 2., 3.));
        app.draw_at(mesh, Vec3::new(1., 1., 12.), Vec3::new(-30., -30., app.user.snd_height), Vec3::new(0., -std::f32::consts::PI/2., std::f32::consts::PI/2.), Vec3::new(2., 2., 3.));
    }
}

fn pong_app(nannou_app: &rend_ox::nannou::App) -> App<Snake> {
    let mut app = app(nannou_app, Snake::new()).update(pong_update);

    if let Ok(mut graphics) = app.graphics.try_borrow_mut() {
        if let Ok(md) = graphics.load_mesh("./ball.obj") {
            app.user.ball = Some(md);
        } else {
            println!("Error loading ball!")
        }
        if let Ok(md) = graphics.load_mesh("./rack.obj") {
            app.user.rack = Some(md);
        } else {
            println!("Error loading rack!")
        }
    } else {
        println!("Error!")
    }
    app
}

fn main() {
    rend_ox::app::launch_rendox_app(pong_app);
}