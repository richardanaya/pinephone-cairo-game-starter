mod engine;
use cairo::{Context, ImageSurface};
use std::cell::RefCell;
use std::rc::Rc;

struct Game {
    init: bool,
    opponent_paddle_x: f64,
    opponent_paddle_y: f64,
    player_paddle_x: f64,
    player_paddle_y: f64,
    ball_x: f64,
    ball_y: f64,
    ball_vel_x: f64,
    ball_vel_y: f64,
}

fn clear_screen(ctx: &Context, r: f64, g: f64, b: f64) {
    ctx.set_source_rgb(r, g, b);
    ctx.paint();
}

fn draw_image_centered(ctx: &Context, x: f64, y: f64, img: &ImageSurface) {
    ctx.save();
    ctx.translate(
        x - (img.get_width() / 2) as f64,
        y - (img.get_height() / 2) as f64,
    );
    ctx.set_source_surface(img, 0.0, 0.0);
    ctx.paint();
    ctx.restore();
}

fn main() {
    engine::init();
    let game = Rc::new(RefCell::new(Game {
        init: false,
        opponent_paddle_x: 0.0,
        opponent_paddle_y: 0.0,
        player_paddle_x: 0.0,
        player_paddle_y: 0.0,
        ball_x: 0.0,
        ball_y: 0.0,
        ball_vel_x: engine::random() * 400.0 - 200.0,
        ball_vel_y: 200.0 * engine::random_sign(),
    }));

    let img_ball = engine::image_from_resource("/app/ball.png");
    let img_paddle = engine::image_from_resource("/app/paddle.png");

    engine::run_game(move |window, ctx, pointer, delta_time| {
        let mut g = game.borrow_mut();
        if !g.init {
            g.opponent_paddle_x = window.width / 2.0;
            g.opponent_paddle_y = 50.0;
            g.player_paddle_x = window.width / 2.0;
            g.player_paddle_y = window.height - 50.0;
            g.ball_x = window.width / 2.0;
            g.ball_y = window.height / 2.0;
            g.init = true;
        }

        g.ball_x += g.ball_vel_x * delta_time;
        g.ball_y += g.ball_vel_y * delta_time;
        g.opponent_paddle_x = g.ball_x;

        if g.ball_y < 50.0 || ( (g.ball_y > window.height - 50.0) && (g.ball_x>g.player_paddle_x-(img_paddle.get_width()/2) as f64 && g.ball_x<g.player_paddle_x+(img_paddle.get_width()/2) as f64)) {
            g.ball_vel_y *= -1.0;
        }

        if g.ball_x < 0.0 || g.ball_x > window.width {
            g.ball_vel_x *= -1.0;
        }

        if g.ball_y > window.height {
            g.ball_x = window.width / 2.0;
            g.ball_y = window.height / 2.0;
            g.ball_vel_x = engine::random() * 400.0 - 200.0;
            g.ball_vel_y = 200.0 * engine::random_sign();
        }

        clear_screen(ctx, 1.0, 1.0, 1.0);

        if pointer.is_down {
            g.player_paddle_x = pointer.x;
        }

        draw_image_centered(ctx, g.ball_x, g.ball_y, &img_ball);
        draw_image_centered(ctx, g.opponent_paddle_x, g.opponent_paddle_y, &img_paddle);
        draw_image_centered(ctx, g.player_paddle_x, g.player_paddle_y, &img_paddle);
    });
}
