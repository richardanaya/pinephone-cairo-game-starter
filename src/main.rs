use std::f64::consts::PI;
mod engine;

struct Game {
    x: f64,
    y: f64,
}

fn main() {
    engine::load_resources();
    let g = Game { x: 0.0, y: 0.0 };
    let s = engine::image_from_resource("/app/ball.png");
    engine::run_game(move |window, ctx, pointer| {
        if pointer.is_down {
            ctx.set_source_rgba(0.0, 1.0,0.0, 1.0);
        } else {
            ctx.set_source_rgba(1.0, 0.0,0.0, 1.0);
        }
        ctx.arc(window.width/2.0, window.height/2.0, window.width/6.0, 0.0, PI * 2.);
        ctx.fill();
        ctx.set_source_surface(&s,0.0,0.0);
        ctx.paint();
    });
}
