use sdl2::keyboard::Keycode;

pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn press(&mut self, key: Keycode, state: bool) {
        println!("{:?}, {:?}", key, state);
        match key {
            Keycode::Num1 => self.set_key(0x1, state),
            Keycode::Num2 => self.set_key(0x2, state),
            Keycode::Num3 => self.set_key(0x3, state),
            Keycode::Num4 => self.set_key(0xc, state),
            Keycode::Q => self.set_key(0x4, state),
            Keycode::W => self.set_key(0x5, state),
            Keycode::E => self.set_key(0x6, state),
            Keycode::R => self.set_key(0xd, state),
            Keycode::A => self.set_key(0x7, state),
            Keycode::S => self.set_key(0x8, state),
            Keycode::D => self.set_key(0x9, state),
            Keycode::F => self.set_key(0xe, state),
            Keycode::Z => self.set_key(0xa, state),
            Keycode::X => self.set_key(0x0, state),
            Keycode::C => self.set_key(0xb, state),
            Keycode::V => self.set_key(0xf, state),
            _ => (),
        }
    }

    fn set_key(&mut self, index: usize, state: bool) {
        self.keys[index] = state;
    }
}
