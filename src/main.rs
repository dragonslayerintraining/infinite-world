mod infinite_grid_mis;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState{
        cache : infinite_grid_mis::Cache,
}

impl MainState{
    fn new() -> GameResult<MainState> {
        Ok(MainState{cache : infinite_grid_mis::Cache::new(),})
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::clear(ctx, [0.1,0.2,0.0,1.0].into());
        let green = graphics::Color::new(0.0,1.0,0.0,1.0);
        let circle = graphics::Mesh::new_circle(ctx,graphics::DrawMode::fill(),
                                                 na::Point2::new(0.0,0.0),
                                                 10.0,2.0,green,)?;
        graphics::draw(ctx,&circle,(na::Point2::new(0.0,0.0),))?;
        
        for x in 0..100 {
            for y in 0..100 {
                match self.cache.get(x,y) {
                    true => graphics::draw(ctx,&circle,(na::Point2::new(10.0*(x as f32),10.0*(y as f32)),))?,
                    false => (),
                };
            }
        }
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("infinte_world","dragonslayerintraining");
    let (ctx,event_loop)=&mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx,event_loop,state)
}


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
