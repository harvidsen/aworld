use comfy::*;

const WIDTH: f32 = 15.0; // Default camera zoom is 30, https://comfyengine.org/book/camera/
const GROUND: f32 = -5.0; // Ground level, so we can draw a line at the bottom of the
const GROUND_THICKNESS: f32 = 0.1;
const GUY_SIZE: f32 = 0.5; // Size of the guy (wow, thanks Copilot)

comfy_game!("AWorld", AWorld);

pub struct Soul {
   pub x: f32,
   pub y: f32,
}

pub struct AWorld {
   pub guy: Soul,
}

impl GameLoop for AWorld {
   fn new(_c: &mut EngineState) -> Self {
      Self { guy: Soul { x: 0.0, y: 0.0 } }
   }

   fn update(&mut self, _c: &mut EngineContext) {
      draw_line(
         vec2(-WIDTH, GROUND),
         vec2(WIDTH, GROUND),
         GROUND_THICKNESS,
         WHITE,
         0
      );
      draw_circle(vec2(self.guy.x, self.guy.y), GUY_SIZE, RED, 0);

      if self.guy.y > GROUND + GUY_SIZE + GROUND_THICKNESS {
         self.guy.y -= 0.1;
      }

      if is_key_down(KeyCode::Right) {
         self.guy.x += 0.5;
      }
      if is_key_down(KeyCode::Left) {
         self.guy.x -= 0.5;
      }
      if is_key_pressed(KeyCode::Space) {
         self.guy.y += 1.0;
      }
   }
}
