pub mod display;
pub mod input;

pub mod game {
    use std::process;
    use dyn_clone::{clone_trait_object, DynClone};

    use sdl2::event::Event;
    use sdl2::EventPump;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;

    use crate::display::{Display, VoidDisplay};
    use crate::input::{Input, InputMap, VoidInput};

    pub trait Update {
        fn update(&mut self, scene: &Scene);
    }

    struct VoidUpdate { }
    impl Update for VoidUpdate {
        fn update(&mut self, _: &Scene) { }
    }

    pub trait Collider {
        fn collider(&self) -> Option<Rect>;
    }

    struct VoidCollider { }

    impl Collider for VoidCollider {
        fn collider(&self) -> Option<Rect> { None }
    }

    pub struct BoxCollider {
        pub x: i32,
        pub y: i32,
        pub width: u32,
        pub height: u32,
    }

    impl Collider for BoxCollider {
        fn collider(&self) -> Option<Rect> {
            Some(Rect::new(self.x, self.y, self.width, self.height))
        }
    }

    pub trait Entity : DynClone {
        fn display(&self) -> Box<dyn Display> {
            Box::new(VoidDisplay {})
        }

        fn input(&mut self) -> Box<dyn Input + '_> {
            Box::new(VoidInput {})
        }

        fn update(&mut self) -> Box<dyn Update + '_> {
            Box::new(VoidUpdate {})
        }

        fn collider(&self) -> Box<dyn Collider> {
            Box::new(VoidCollider {})
        }
    }

    clone_trait_object!(Entity);


    #[derive(Clone)]
    pub struct Scene {
        pub entities: Vec<Box<dyn Entity>>,
    }

    impl Scene {
        pub fn new() -> Self {
            Scene {
                entities: vec![],
            }
        }

        pub fn add(&mut self, entity: Box<dyn Entity>) {
            self.entities.push(entity);
        }
    }

    pub struct Game {
        canvas: WindowCanvas,
        event_pump: EventPump,
        scene: Option<Scene>,
    }

    impl Game {
        pub fn init(title: &str) -> Game {
            let sdl = sdl2::init().unwrap_or_else(|_| { panic!("Can't initialize SDL2.") });
            let video = sdl.video().unwrap_or_else(|_| { panic!("Can't initialize video.") });
            let window = video.window(title, 1280, 720).position_centered().build().unwrap_or_else(|_| { panic!("Can't create window.") });
            let canvas = window.into_canvas().present_vsync().build().expect("Cant create canvas.");
            let event_pump = sdl.event_pump().expect("Cant get event pump.");

            Game {
                canvas,
                event_pump,
                scene: None,
            }
        }

        pub fn set_scene(&mut self, scene: Scene) {
            self.scene = Some(scene);
        }

        pub fn run(&mut self) {
            if self.scene.is_none() {
                panic!("No scene provided");
            }

            loop {
                self.handle_input();
                self.handle_update();

                self.handle_draw();
            }
        }

        fn handle_input(&mut self) {
            for event in self.event_pump.poll_iter() {
                match &event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => process::exit(0),
                    _ => {}
                }
            }

            let state = InputMap { keyboard: self.event_pump.keyboard_state() };

            for entity in &mut self.scene.as_mut().unwrap().entities {
                entity.input().input(&state);
            }
        }

        fn handle_update(&mut self) {
            let previous = self.scene.as_ref().unwrap().clone();

            for entity in &mut self.scene.as_mut().unwrap().entities {
                entity.update().update(&previous);
            }
        }

        fn handle_draw(&mut self) {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            for entity in &self.scene.as_ref().unwrap().entities {
                entity.display().display(&mut self.canvas);
            }

            self.canvas.present();
        }
    }
}