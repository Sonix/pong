use sdl2::keyboard::{Scancode};
use sdl2::pixels::Color;
use pong::display::{Circle, Display, Rect, ScreenFill};
use pong::game::{Game, Entity, Scene, Update, Collider, BoxCollider};
use pong::input::{Input, InputMap};


#[derive(Clone)]
struct Background {}

impl Entity for Background {
    fn display(&self) -> Box<dyn Display> {
        Box::new(ScreenFill{color: Color::YELLOW})
    }
}

#[derive(PartialEq, Clone)]
enum Player {
    Left,
    Right,
}

#[derive(Clone)]
struct Paddle {
    pos: i32,
    player: Player,
}

impl Entity for Paddle {
    fn display(&self) -> Box<dyn Display> {
        Box::new(Rect{
            color: if self.player == Player::Left {Color::BLUE} else {Color::RED},
            x: if self.player == Player::Left {50} else {1280-50-20},
            y: self.pos,
            width: 20,
            height: 150,
        })
    }

    fn input(&mut self) -> Box<dyn Input + '_> {
        struct PaddleInput<'a> {
            target: &'a mut Paddle,
        }

        impl Input for PaddleInput<'_> {
            fn input(&mut self, input: &InputMap) {
                const SPEED: i32 = 10;
                if (input.keyboard.is_scancode_pressed(Scancode::W) && self.target.player == Player::Left) || (input.keyboard.is_scancode_pressed(Scancode::Up) && self.target.player == Player::Right) {
                    self.target.pos -= SPEED;
                }
                if (input.keyboard.is_scancode_pressed(Scancode::S) && self.target.player == Player::Left) || (input.keyboard.is_scancode_pressed(Scancode::Down) && self.target.player == Player::Right)  {
                    self.target.pos += SPEED;
                }
            }
        }

        Box::new(PaddleInput{target: self})
    }

    fn collider(&self) -> Box<dyn Collider> {
        Box::new(BoxCollider {
            x: if self.player == Player::Left {50} else {1280-50-20},
            y: self.pos,
            width: 20,
            height: 150,
        })
    }
}

#[derive(Clone)]
struct Ball {
    x: i32,
    y: i32,
    direction: (i32, i32),
}

const BALL_RADIUS: i32 = 10;

impl Entity for Ball {
    fn display(&self) -> Box<dyn Display> {
        Box::new(Circle{x: self.x,y: self.y, radius: BALL_RADIUS as u32, color: Color::GREEN, filled: true})
    }

    fn update(&mut self) -> Box<dyn Update + '_> {
        struct BallUpdate<'a> {
            target: &'a mut Ball,
        }

        impl Update for BallUpdate<'_> {
            fn update(&mut self, scene: &Scene) {
                const SPEED: i32 = 10;
                self.target.x = self.target.x + self.target.direction.0 * SPEED;
                self.target.y = self.target.y + self.target.direction.1 * SPEED;

                if self.target.x - BALL_RADIUS < 0 && self.target.direction.0 < 0 {
                    self.target.x = BALL_RADIUS;
                    self.target.direction.0 *= -1;
                }
                if self.target.x + BALL_RADIUS > 1280 && self.target.direction.0 > 0 {
                    self.target.x = 1280 - BALL_RADIUS;
                    self.target.direction.0 *= -1;
                }
                if self.target.y - BALL_RADIUS < 0 && self.target.direction.1 < 0 {
                    self.target.y = BALL_RADIUS;
                    self.target.direction.1 *= -1;
                }
                if self.target.y + BALL_RADIUS > 720 && self.target.direction.1 > 0 {
                    self.target.y = 720 - BALL_RADIUS;
                    self.target.direction.1 *= -1;
                }

                let collider = sdl2::rect::Rect::new(self.target.x - BALL_RADIUS, self.target.y - BALL_RADIUS,(BALL_RADIUS * 2) as u32, (BALL_RADIUS * 2) as u32);
                for entity in &scene.entities {
                    match entity.collider().collider() {
                        Some(other) => {
                            if collider.has_intersection(other) {
                                self.target.direction.0 *= -1;
                            }
                        }
                        _ => {}
                    }
                }

            }
        }

        Box::new(BallUpdate {target: self})
    }
}

fn main() {
    let mut game = Game::init("Pong");

    let mut world = Scene::new();
    world.add(Box::new(Background{}));
    let left = Box::new(Paddle { player: Player::Left, pos: 300 });
    world.add(left);
    let right = Box::new(Paddle { player: Player::Right, pos: 300 });
    world.add(right);
    let ball = Box::new(Ball{x: 1280 / 2, y: 720 / 2, direction: (1, 1)});
    world.add(ball);

    game.set_scene(world);

    game.run();
}
