use comfy::*;

const WIDTH: f32 = 15.0; // Default camera zoom is 30, https://comfyengine.org/book/camera/
const GROUND_LEVEL: f32 = -5.0; // Main ground level
const GROUND_THICKNESS: f32 = 0.1;
const JUMP_FACTOR: f32 = 1.0; // How much we are able to jump
const FALL_FACTOR: f32 = 0.1; // I.e. how strong the gravity is
const MAX_FALL_SPEED: f32 = 10.0; // I.e. terminal velocity

comfy_game!("AWorld", AWorld);

trait Acrobatics {
   fn jump(&mut self);
   fn land(&mut self);
   fn iter_air(&mut self, land: &Land); // TODO: Can we make this not require land?
   fn on_ground(&self, land: &Land) -> bool;
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
   fn on_ground(&self, land: &Land) -> bool { // TODO: More logical to put this in ground or land?
      if self.y_speed > 0.0 { return false; } // Don't abort jumps

      for ground in &land.pieces {
         if self.x > ground.x_left // TODO: Sould should have a width to not fall over edge too easily
            && self.x < ground.x_right
            // Need to account for y_speed to not fall past floors
            && self.y - self.height + self.y_speed <= ground.y + GROUND_THICKNESS // If bottom on/under ground
            && self.y - self.y_speed >= ground.y { // If middle over ground
            println!("On ground {:?}", ground);
            return true
         }
      }
      return false
   }
   // Iterate vertical movement that should happen while airborne
   fn iter_air(&mut self, land: &Land) {
      println!("iter_air {:?}", self);
      self.y_speed -= 1.0 * FALL_FACTOR;
      self.y += (-MAX_FALL_SPEED).max(self.y_speed);

      if self.on_ground(land) {
         println!("Landing\n");
         self.land()
      }
   }
}

// One piece of the land

#[derive(Debug)]
pub struct Ground {
   x_left: f32,
   x_right: f32,
   y: f32,
}

impl Ground {
   fn draw(&self) {
      draw_line(
         vec2(self.x_left, self.y),
         vec2(self.x_right, self.y),
         GROUND_THICKNESS,
         WHITE,
         0,
      )
   }
}

fn make_ground (pos: Vec2, width: f32) -> Ground {
   Ground {
      x_left: pos.x - width,
      x_right: pos.x + width,
      y: pos.y
   }
}

// The entire land we can explore
pub struct Land {
   pieces: [Ground; 4],
}

impl Land {
   fn draw(&self) {
      for ground in &self.pieces {
         ground.draw()
      }
   }
}

pub struct AWorld {
   pub guy: Soul,
   pub land: Land
}

impl GameLoop for AWorld {
   fn new(_c: &mut EngineState) -> Self {
      Self {
         guy: Soul { x: 0.0, y: 0.0, height: 0.5, y_speed: 0.0 },
         land: Land {
            pieces: [
               make_ground(vec2(0.0, GROUND_LEVEL), WIDTH),
               make_ground(vec2(0.0, GROUND_LEVEL + 10.0), 3.0 ),
               make_ground(vec2(-5.0, GROUND_LEVEL + 5.0), 3.0 ),
               make_ground(vec2(5.0, GROUND_LEVEL + 5.0), 3.0 ),
            ]
         }
      }
   }

   fn update(&mut self, _c: &mut EngineContext) {
      self.land.draw();

      // TODO: Put drawing in to Soul
      draw_circle(vec2(self.guy.x, self.guy.y), self.guy.height, RED, 0);

      if !self.guy.on_ground(&self.land) {
         self.guy.iter_air(&self.land);
      };

      if is_key_down(KeyCode::Right) {
         self.guy.x += 0.5;
      }
      if is_key_down(KeyCode::Left) {
         self.guy.x -= 0.5;
      }
      if is_key_pressed(KeyCode::Space) {
         if self.guy.on_ground(&self.land) {
            println!("Jumping {:?}", self.guy);
            self.guy.jump()
         }
      }
   }
}
