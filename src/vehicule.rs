use std::time:: Instant;

const CAR_WIDTH: u32 = 25;
const CAR_HEIGHT: u32 = 30;
const UPDATE_THRESHOLD: u32 = 10;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
pub struct Vehicule<'a> {
    pub texture: &'a sdl2::render::Texture<'a>,
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub speed: i32,
    pub distance: i32,
    pub time: i32,
    pub timer: Instant,
    pub states: bool,
    pub frame_count: u32,
    pub angle: f64,
    pub cote:i32,
    pub turning: bool, 
}

impl<'a> Vehicule<'a> {
    pub fn new(
        texture: &'a sdl2::render::Texture<'a>,
        x: i32,
        y: i32,
        direction: Direction,
        angle: f64,
        cote : i32,
    ) -> Self {
        Vehicule {
            texture,
            x,
            y,
            direction,
            speed: 3,
            distance: 0,
            time: 0,
            timer: Instant::now(),
            states: true,
            frame_count: 0,
            angle, 
            cote ,
            turning: false,
        }
    }
    pub fn collides_with(&self, other: &Vehicule, safe_distance: i32) -> bool {
    
        match self.direction {
            Direction::Down => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.y >= self.y {
                    return dy <= safe_distance as i32 && dx < (CAR_WIDTH as i32);
                }
            }
            Direction::Up => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.y >= other.y {
                    return dy <= safe_distance as i32 && dx < (CAR_WIDTH as i32);
                }
            }
            Direction::Left => {
                let dx = (self.x - other.x).abs();
                let dy = (self.y - other.y).abs();
                if self.x >= other.x {
                    return dx <= safe_distance as i32 && dy < (CAR_HEIGHT as i32);
                }
            }
            Direction::Right => {
                let dx = (other.x - self.x).abs();
                let dy = (other.y - self.y).abs();
                if other.x >= self.x {
                    return dx <= safe_distance as i32 && dy < (CAR_HEIGHT as i32);
                }
            }
        }
        false
    }

    pub fn should_turning(&self)-> Option<Direction> {
        match self.direction {
            Direction::Up => {
                if self.turning {
                    if self.y <= 355 && self.angle == 0.0 && self.x == 410 {
                        return  Some(Direction::Left);
                    } else if self.y <= 490 && self.angle == 0.0 && self.x == 500 {
                        return  Some(Direction::Right);
                    }
                }
                None
            }
            Direction::Down => {
                if self.turning {
                    if self.y >= 405 && self.angle == 180.0 && self.x == 365 {
                        return  Some(Direction::Right);
                        
                    } else if self.y >= 270 && self.angle == 180.0 && self.x == 275 {
                        return  Some(Direction::Left);
                    }
                }
                None
            }
            Direction::Left => {
                if self.turning {
                    if self.y == 270 && self.angle == -90.0 && self.x <= 500 {
                        return Some(Direction::Up);
                    } else if self.y == 360 && self.angle == -90.0 && self.x <= 365 {
                        return  Some(Direction::Down);
                    }
                }
                None
            }
            Direction::Right => {
                if self.turning {
                    if self.y == 400 && self.angle == 90.0 && self.x >= 405 {
                        return  Some(Direction::Up);
                    } else if self.y == 490 && self.angle == 90.0 && self.x >= 275 {
                        return  Some(Direction::Down);
                    }
                }
                None
            }
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.turning {
                    if self.y <= 355 && self.angle == 0.0 && self.x == 410 {
                        self.direction = Direction::Left;
                        self.angle = -90.0;
                        self.turning=false;

                        self.speed = 0;
                    } else if self.y <= 490 && self.angle == 0.0 && self.x == 500 {
                        self.direction = Direction::Right;
                        self.angle = 90.0;
                        self.turning=false;
                        self.speed = 0;
                    }
                }
                self.y -= self.speed;
            }
            Direction::Down => {
                if self.turning {
                    if self.y >= 405 && self.angle == 180.0 && self.x == 365 {
                        self.direction = Direction::Right;
                        self.angle = 90.0;
                        self.turning=false;
                        self.speed = 0;
                    } else if self.y >= 270 && self.angle == 180.0 && self.x == 275 {
                        self.direction = Direction::Left;
                        self.angle = -90.0;
                        self.turning=false;
                        self.speed = 0;
                    }
                }
                self.y += self.speed;
            }
            Direction::Left => {
                if self.turning {
                    if self.y == 270 && self.angle == -90.0 && self.x <= 500 {
                        self.direction = Direction::Up;
                        self.angle = 0.0;
                        self.turning=false;
                        self.speed = 0; 
                    } else if self.y == 360 && self.angle == -90.0 && self.x <= 365 {
                        self.direction = Direction::Down;
                        self.angle = 180.0;
                        self.turning=false;
                        self.speed = 0;
                    }
                }
                self.x -= self.speed;
            }
            Direction::Right => {
                if self.turning {
                    if self.y == 400 && self.angle == 90.0 && self.x >= 405 {
                        self.direction = Direction::Up;
                        self.angle = 0.0;
                        self.turning=false;
                        self.speed = 0;
                    } else if self.y == 490 && self.angle == 90.0 && self.x >= 275 {
                        self.direction = Direction::Down;
                        self.angle = 180.0;
                        self.turning=false;
                        self.speed = 0;
                    }
                }
                self.x += self.speed;
            }
        }

        self.distance += self.speed;
        self.time += 1;
    }
    pub fn should_update(&mut self) -> bool {
        self.frame_count >= UPDATE_THRESHOLD
    }

    pub fn reset_frame_count(&mut self) {
        self.frame_count = 0;
    }

    pub fn increment_frame_count(&mut self) {
        self.frame_count += 1;
    }
}