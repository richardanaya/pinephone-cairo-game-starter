use rand::Rng;
use std::f64::consts::PI;

mod setup;

fn main() {
    setup::run_game(|ctx, touch| {
        let mut rng = rand::thread_rng();
        ctx.scale(400f64, 400f64);
        let rgb = (rng.gen(), rng.gen(), rng.gen());
        ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
        ctx.arc(0.40, 0.53, 0.2, 0.0, PI * 2.);
        ctx.fill();

        if touch.is_down {
            let rgb = (rng.gen(), rng.gen(), rng.gen());
            ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
            ctx.arc(0.5, 0.65, 0.2, 0.0, PI * 2.);
            ctx.fill();
            let rgb = (rng.gen(), rng.gen(), rng.gen());
            ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
            ctx.arc(0.6, 0.53, 0.2, 0.0, PI * 2.);
            ctx.fill();
        }
    });
}
