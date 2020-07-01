mod infinite_grid_mis;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState{
        cache : infinite_grid_mis::Cache,
        camera : na::Point2<f32>,
}

impl MainState{
    fn new() -> GameResult<MainState> {
        Ok(MainState{
            cache : infinite_grid_mis::Cache::new(),
            camera : na::Point2::new(-100.0,-100.0),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.camera+=na::Vector2::new(1.0,1.0);
        Ok(())
    }
    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::clear(ctx, [0.1,0.2,0.0,1.0].into());
        let green = graphics::Color::new(0.0,1.0,0.0,1.0);
        let circle = graphics::Mesh::new_circle(ctx,graphics::DrawMode::fill(),
                                                 na::Point2::new(0.0,0.0),
                                                 18.0,2.0,green,)?;
        for x in 0..50 {
            for y in 0..50 {
                match self.cache.get(x,y) {
                    true => {
                        let pos = na::Point2::new(20.0*(x as f32),20.0*(y as f32));
                        let rel_pos = na::Point2::origin()+(pos-self.camera);
                        graphics::draw(ctx,&circle,(rel_pos,))?;
                    },
                    false => (),
                };
            }
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
    
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("infinte_world","dragonslayerintraining")
        .window_setup(ggez::conf::WindowSetup::default().title("An infinite world"))
        ;//.window_mode(ggez::conf::WindowMode::default().dimensions(500.0,500.0));
    let (ctx,event_loop)=&mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx,event_loop,state)
}

//Output world with ANSI escape codes
fn ansi_main() {
    let mut cache : infinite_grid_mis::Cache = infinite_grid_mis::Cache::new();
    for x in 0..100 {
        for y in 0..100 {
            let s: String = match cache.get(x,y) {
                true => "\x1b[42m  \x1b[49m".to_string(),
                false => "\x1b[40m  \x1b[49m".to_string(),
            };
            print!("{}",s);
        }
        println!();
    }
    //println!("Cache size: {}",cache.len());
}
