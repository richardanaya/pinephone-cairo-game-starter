use std::f64::consts::PI;
mod engine;
use std::cell::RefCell;
use std::rc::Rc;

struct Game {
    x: f64,
    y: f64,
}

fn main() {
    engine::load_resources();
    let game = Rc::new(RefCell::new(Game { x: 0.0, y: 0.0 }));
    let s = engine::image_from_resource("/app/ball.png");
    engine::run_game(move |window, ctx, pointer, delta_time| {
        let mut g = game.borrow_mut();
        if pointer.is_down {
            g.x = pointer.x;
            g.y = pointer.y;
        }
        if pointer.is_down {
            ctx.set_source_rgba(0.0, 1.0, 0.0, 1.0);
        } else {
            ctx.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        }
        ctx.arc(g.x, g.y, window.width / 6.0, 0.0, PI * 2.);
        ctx.fill();
        ctx.set_source_surface(&s, 0.0, 0.0);
        ctx.paint();
    });
}
