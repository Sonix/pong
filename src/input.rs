use sdl2::keyboard::KeyboardState;

pub struct InputMap<'a> {
    pub keyboard: KeyboardState<'a>,
}

pub trait Input {
    fn input(&mut self, input: &InputMap);
}


pub struct VoidInput {}

impl Input for VoidInput {
    fn input(&mut self, _: &InputMap) {}
}
