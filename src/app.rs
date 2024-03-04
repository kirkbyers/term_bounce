#[derive(Debug, Default)]
pub struct App {
    pub ball_size: (u16, u16), // size is 2d radius
    pub ball_pos: (u16, u16),
    pub ball_velocity: (i16, i16),
    pub ball_bounds: (u16, u16), // Upper bounds of playspace
    pub size_increment: (u16, u16),
}

impl App {
    pub fn new() -> App {
        App::new_with_values(10, 10)
    }

    pub fn new_with_values(x: u16, y: u16) -> App {
        App {
            ball_size: (2, 1),
            ball_pos: (x, y),
            ball_velocity: (4, 2),
            ball_bounds: (100, 100),
            size_increment: (2, 1),
        }
    }

    pub fn update_bounds(&mut self, x: u16, y: u16) {
        self.ball_bounds = (x, y);
    }

    pub fn update_center(&mut self) {
        let (mut x, mut y) = self.ball_pos;
        let (vx, vy) = self.ball_velocity;
        let (sx, sy) = self.ball_size;
        let (bx, by) = self.ball_bounds;

        let px = x.checked_add_signed(vx);
        let py = y.checked_add_signed(vy);
        let mut did_collide = [false, false];
        // Check for collision with the bounds
        match px {
            Some(px) => {
                if px + sx  > bx {
                    x = bx - sx - self.size_increment.0;
                    did_collide[0] = true;
                } else {
                    x = px;
                }
            }
            None => {
                x = 0;
                did_collide[0] = true;
            }
        }
        match py {
            Some(py) => {
                if py + sy > by {
                    y = by - sy - self.size_increment.1;
                    did_collide[1] = true;
                } else {
                    y = py;
                }
            }
            None => {
                y = 0;
                did_collide[1] = true;
            }
        }

        if did_collide.iter().any(|&x| x) {
            self.ball_size = (sx+self.size_increment.0, sy+self.size_increment.1);
            if self.ball_size.0 > bx {
                self.ball_size.0 = self.size_increment.0;
                self.ball_size.1 = self.size_increment.1;
            }
            if self.ball_size.1 > by {
                self.ball_size.0 = self.size_increment.0;
                self.ball_size.1 = self.size_increment.1;
            }
            // Reverse the velocity
            if did_collide[0] {
                self.ball_velocity.0 = -vx;
            }
            if did_collide[1] {
                self.ball_velocity.1 = -vy;
            }
        }

        self.ball_pos = (x, y);
    }
}