use comfy::*;

mod soul;
mod world;
mod c;
use soul::Soul;
use world::{
   World,
   make_ground,
};


comfy_game!("AWorld", AWorld);

pub struct AWorld {
   pub guy: Soul,
   pub world: World
}

impl GameLoop for AWorld {
   fn new(_c: &mut EngineState) -> Self {
      Self {
         guy: Soul { x: 0.0, y: 0.0, height: 0.5, y_speed: 0.0, in_air: true },
         world: World {
            pieces: [
               make_ground(vec2(0.0, c::GROUND_LEVEL), c::WIDTH),
               make_ground(vec2(0.0, c::GROUND_LEVEL + 10.0), 3.0 ),
               make_ground(vec2(-5.0, c::GROUND_LEVEL + 5.0), 3.0 ),
               make_ground(vec2(5.0, c::GROUND_LEVEL + 5.0), 3.0 ),
            ]
         }
      }
   }

   fn update(&mut self, _c: &mut EngineContext) {
      // println!("update");
      self.world.draw();

      // TODO: Put drawing in to Soul
      draw_circle(vec2(self.guy.x, self.guy.y), self.guy.height, RED, 0);

      self.guy.on_ground(&self.world);

      if self.guy.in_air {
         self.guy.iter_air();
      };

      if is_key_down(KeyCode::Right) {
         self.guy.x += 0.5;
      }
      if is_key_down(KeyCode::Left) {
         self.guy.x -= 0.5;
      }
      if is_key_pressed(KeyCode::Space) {
         self.guy.jump()
      }
   }
}
