use comfy::*;

const WIDTH: f32 = 15.0; // Default camera zoom is 30, https://comfyengine.org/book/camera/
const GROUND: f32 = -5.0; // Ground level, so we can draw a line at the bottom of the
const GROUND_THICKNESS: f32 = 0.1;
const JUMP_FACTOR: f32 = 1.0; // How much we are able to jump
const FALL_FACTOR: f32 = 0.1; // I.e. how strong the gravity is
const MAX_FALL_SPEED: f32 = 10.0; // I.e. terminal velocity

comfy_game!("AWorld", AWorld);

trait Acrobatics {
   fn jump(&mut self);
   fn land(&mut self);
   fn iter_air(&mut self);
   fn on_ground(&self) -> bool;
}

#[derive(Debug)]
pub struct Soul {
   pub x: f32,
   pub y: f32,
   pub height: f32,
   pub y_speed: f32,
}

impl Acrobatics for Soul {
   fn jump(&mut self) {
      self.y_speed += 1.0 * JUMP_FACTOR;
      self.y += self.y_speed;
   }
   fn land(&mut self) {
      self.y_speed = 0.0;
   }
   fn on_ground(&self) -> bool {
      self.y - self.height < GROUND + GROUND_THICKNESS
   }
   // Iterate vertical movement that should happen while airborne
   fn iter_air(&mut self) {
      println!("iter_air {:?}", self);
      self.y_speed -= 1.0 * FALL_FACTOR;
      self.y += (-MAX_FALL_SPEED).max(self.y_speed);

      if self.on_ground() {
         println!("Landing\n");
         self.land()
      }
   }
}


pub struct AWorld {
   pub guy: Soul,
}

impl GameLoop for AWorld {
   fn new(_c: &mut EngineState) -> Self {
      Self { guy: Soul { x: 0.0, y: 0.0, height: 0.5, y_speed: 0.0 } }
   }

   fn update(&mut self, _c: &mut EngineContext) {
      draw_line(
         vec2(-WIDTH, GROUND),
         vec2(WIDTH, GROUND),
         GROUND_THICKNESS,
         WHITE,
         0
      );

      // TODO: Put drawing in to Soul
      draw_circle(vec2(self.guy.x, self.guy.y), self.guy.height, RED, 0);

      if !self.guy.on_ground() {
         self.guy.iter_air();
      };

      if is_key_down(KeyCode::Right) {
         self.guy.x += 0.5;
      }
      if is_key_down(KeyCode::Left) {
         self.guy.x -= 0.5;
      }
      if is_key_pressed(KeyCode::Space) {
         if self.guy.on_ground() {
            println!("Jumping {:?}", self.guy);
            self.guy.jump()
         }
      }
   }
}
