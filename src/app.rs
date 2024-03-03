#[derive(Debug, Default)]
pub struct App {
    pub ball_size: (u16, u16), // size is 2d radius
    pub ball_center: (u16, u16),
    pub ball_velocity: (i16, i16),
    pub ball_bounds: (u16, u16), // Upper bounds of playspace
    pub size_increment: u16,
}

impl App {
    pub fn new() -> App {
        App::new_with_values(10, 10)
    }

    pub fn new_with_values(x: u16, y: u16) -> App {
        App {
            ball_size: (1, 1),
            ball_center: (x, y),
            ball_velocity: (1, 1),
            ball_bounds: (100, 100),
            size_increment: 1,
        }
    }

    pub fn update_bounds(&mut self, x: u16, y: u16) {
        self.ball_bounds = (x, y);
    }

    pub fn update_center(&mut self) {
        let (mut x, mut y) = self.ball_center;
        let (vx, vy) = self.ball_velocity;
        let (sx, sy) = self.ball_size;
        let (bx, by) = self.ball_bounds;

        let px = x.checked_add_signed(vx);
        let py = y.checked_add_signed(vy);
        let mut did_collide = [false, false];
        // Check for collision with the bounds
        match px {
            Some(px) => {
                if px  > bx {
                    x = bx;
                    did_collide[0] = true;
                } else if px == 0 {
                    x = 0;
                    did_collide[0] = true;
                } else {
                    x = px;
                }
                // Check size fits within the bounds
                if px + sx > bx {
                    x = bx - sx;
                    did_collide[0] = true;
                }
                if px.checked_sub(sx).is_none() {
                    x = 0 + sx;
                    did_collide[0] = true;
                }
            }
            None => {
                x = 0 + sx;
                did_collide[0] = true;
            }
        }

        match py {
            Some(py) => {
                if py > by {
                    y = by;
                    did_collide[1] = true;
                } else if py == 0 {
                    y = 0;
                    did_collide[1] = true;
                } else {
                    y = py;
                }
                // Check size fits within the bounds
                if py + sy > by {
                    y = by - sy;
                    did_collide[1] = true;
                }
                if py.checked_sub(sy).is_none() {
                    y = 0 + sy;
                    did_collide[1] = true;
                }
            }
            None => {
                y = 0 + sy;
                did_collide[1] = true;
            }
        }

        if did_collide.iter().any(|&x| x) {
            self.ball_size = (sx+self.size_increment, sy+self.size_increment);
            if self.ball_size.0 > bx {
                self.ball_size.0 = self.size_increment;
            }
            if self.ball_size.1 > by {
                self.ball_size.1 = self.size_increment;
            }
            // Reverse the velocity
            if did_collide[0] {
                self.ball_velocity.0 = -vx;
            }
            if did_collide[1] {
                self.ball_velocity.1 = -vy;
            }
        }

        self.ball_center = (x, y);
    }
}