use comfy::*;

//mod c;
mod world;
use world::Ground;
use c;

#[derive(Debug)]
pub struct Soul {
   pub x: f32,
   pub y: f32,
   pub height: f32,
   pub y_speed: f32,
   pub in_air: bool,
}

trait Acrobatics {
   fn jump(&mut self);
   fn iter_air(&mut self); // TODO: Can we make this not require land?
   fn on_ground(&mut self, land: &World);
   fn land(&mut self, ground: &Ground);
}

impl Acrobatics for Soul {
   fn jump(&mut self) {
      if !self.in_air {
         self.y_speed += 1.0 * c::JUMP_FACTOR;
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
