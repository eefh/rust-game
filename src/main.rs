use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez dt = {}fps", 1.0 / self.dt.as_secs_f32());
        Ok(())
    }
}

pub fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
    };
    println!("Hello, world!");
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "eef")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
