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
   fn iter_air(&mut self); // TODO: Can we make this not require land?
   fn on_ground(&mut self, land: &World);
   fn land(&mut self, ground: &Ground);
}

#[derive(Debug)]
pub struct Soul {
   pub x: f32,
   pub y: f32,
   pub height: f32,
   pub y_speed: f32,
   pub in_air: bool,
}

impl Acrobatics for Soul {
   fn jump(&mut self) {
      if !self.in_air {
         self.y_speed += 1.0 * JUMP_FACTOR;
         self.y += self.y_speed;
      }
   }
   fn land(&mut self, ground: &Ground) {
      self.y_speed = 0.0;
      self.y = ground.y + self.height;
      self.in_air = false;
   }

   fn on_ground(&mut self, land: &World) {
      // Don't abort jumps
      if self.y_speed > 0.0 {
         self.in_air = true;
         return
      }

      // If touching a piece of land
      for ground in &land.pieces {
         if self.x > ground.x_left // TODO: Soul should have a width to not fall over edge too easily
            && self.x < ground.x_right
            // Need to account for y_speed to not fall past floors
            && self.y - self.height + self.y_speed <= ground.y + GROUND_THICKNESS // If bottom on/under ground
            && self.y - self.y_speed >= ground.y { // If middle over ground
            if self.in_air {
               self.land(ground);
            }
            return
         }
      }

      // Otherwise continue to fall
      self.in_air = true;
   }

   // Iterate vertical movement that should happen while airborne
   fn iter_air(&mut self) {
      self.y_speed -= 1.0 * FALL_FACTOR;
      self.y += (-MAX_FALL_SPEED).max(self.y_speed);
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
pub struct World {
   pieces: [Ground; 4],
}

impl World {
   fn draw(&self) {
      for ground in &self.pieces {
         ground.draw()
      }
   }
}

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
               make_ground(vec2(0.0, GROUND_LEVEL), WIDTH),
               make_ground(vec2(0.0, GROUND_LEVEL + 10.0), 3.0 ),
               make_ground(vec2(-5.0, GROUND_LEVEL + 5.0), 3.0 ),
               make_ground(vec2(5.0, GROUND_LEVEL + 5.0), 3.0 ),
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
