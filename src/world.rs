use comfy::*;

// One piece of the land
#[derive(Debug)]
pub struct Ground {
   x_left: f32,
   x_right: f32,
   y: f32,
}

impl Ground {
   pub fn draw(&self) {
      draw_line(
         vec2(self.x_left, self.y),
         vec2(self.x_right, self.y),
         GROUND_THICKNESS,
         WHITE,
         0,
      )
   }
}

pub fn make_ground (pos: Vec2, width: f32) -> Ground {
   Ground {
      x_left: pos.x - width,
      x_right: pos.x + width,
      y: pos.y
   }
}

// The entire land we can explore
pub struct World {
   pub pieces: [Ground; 4],
}

impl World {
   pub fn draw(&self) {
      for ground in &self.pieces {
         ground.draw()
      }
   }
}
