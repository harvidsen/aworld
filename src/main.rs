use comfy::*;

comfy_game!("AWorld", AWorld);

pub struct AWorld {
   pub x: f32,
   pub y: f32,
}

impl GameLoop for AWorld {
   fn new(_c: &mut EngineState) -> Self {
      Self { x: 0.0, y: 0.0 }
   }

   fn update(&mut self, _c: &mut EngineContext) {
      draw_circle(vec2(self.x, self.y), 0.5, RED, 0);


      if is_key_down(KeyCode::Right) {
         self.x += 0.5;
      }
      if is_key_down(KeyCode::Left) {
         self.x -= 0.5;
      }
      if is_key_down(KeyCode::Up) {
         self.y += 0.5;
      }
      if is_key_down(KeyCode::Down) {
         self.y -= 0.5;
      }
   }
}
