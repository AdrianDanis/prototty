extern crate prototty;
extern crate prototty_unix;
extern crate rand;
extern crate tetris_prototty;

use std::time::Duration;
use std::thread;
use prototty::Renderer;
use prototty_unix::Context;
use tetris_prototty::{App, AppView, ControlFlow};

const TICK_MILLIS: u64 = 16;

fn main() {
    let mut context = Context::new().unwrap();
    let mut rng = rand::thread_rng();
    let mut app = App::new(&mut rng);
    let mut app_view = AppView::new();

    loop {
        context.render(&mut app_view, &app).unwrap();
        thread::sleep(Duration::from_millis(TICK_MILLIS));

        if let Some(control_flow) = app.tick(
            context.drain_input().unwrap(),
            Duration::from_millis(TICK_MILLIS),
            &mut rng,
        ) {
            match control_flow {
                ControlFlow::Exit => break,
            }
        }
    }
}
