mod infinite_grid_mis;

use ggez;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState{
        cache : infinite_grid_mis::Cache,
        camera : na::Point2<f32>,
        move_dir : na::Vector2<f32>,
}

impl MainState{
    fn new() -> GameResult<MainState> {
        Ok(MainState{
            cache : infinite_grid_mis::Cache::new(),
            camera : na::Point2::new(-100.0,-100.0),
            move_dir : na::Vector2::new(1.0,0.0),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.camera+=self.move_dir;
        Ok(())
    }
    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::clear(ctx, [0.1,0.2,0.0,1.0].into());

        let green = graphics::Color::new(0.0,1.0,0.0,1.0);
        let green_circle = graphics::Mesh::new_circle(ctx,graphics::DrawMode::fill(),
                                                 na::Point2::new(0.0,0.0),
                                                 18.0,2.0,green,)?;

        let transform = na::geometry::Similarity2::new(na::Point::origin()-self.camera,0.0,20.0);
        let screen_rect = ggez::graphics::screen_coordinates(ctx);

        //Assumes no rotation
        //Transform the screen in reverse to determine which world coordinates are visible.
        let visible_topleft = transform.inverse() * na::Point2::new(screen_rect.left(),screen_rect.top());
        let visible_bottomright = transform.inverse() * na::Point2::new(screen_rect.bottom(),screen_rect.right());
        for x in (visible_topleft.x.floor() as i32)..=(visible_bottomright.x.ceil() as i32) {
            for y in (visible_topleft.y.floor() as i32)..=(visible_bottomright.y.ceil() as i32) {
                let coords = transform * na::Point2::new(x as f32, y as f32);
                match self.cache.get(x,y) {
                    true => {
                        graphics::draw(ctx,&green_circle,(coords,))?;
                    },
                    false => (),
                };
            }
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
    fn key_down_event(&mut self,_ctx:&mut Context,keycode: KeyCode, _keymod: KeyMods,_repeat:bool,){
        match keycode{
            KeyCode::Up => { self.move_dir = na::Vector2::new(0.0,-1.0); }
            KeyCode::Down => { self.move_dir = na::Vector2::new(0.0,1.0); }
            KeyCode::Left => { self.move_dir = na::Vector2::new(-1.0,0.0); }
            KeyCode::Right => { self.move_dir = na::Vector2::new(1.0,0.0); }
            KeyCode::Space => { self.move_dir = na::Vector2::new(0.0,0.0); }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("infinte_world","dragonslayerintraining")
        .window_setup(ggez::conf::WindowSetup::default().title("An infinite world"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0,800.0));
    let (ctx,event_loop)=&mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx,event_loop,state)
}
