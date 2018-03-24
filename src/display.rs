use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;


pub struct Display {
    gfx: [[u8; 64]; 32],
    draw_flag: bool,
    canvas: WindowCanvas,
}

static SCALE: u32 = 20;

impl Display {
    pub fn new(sdl_context: Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8 Emulator", 64 * SCALE, 32 * SCALE)
                    .position_centered()
                    .opengl()
                    .build()
                    .unwrap();
    
        Display {
            gfx: [[0; 64]; 32],
            draw_flag: true,
            canvas: window.into_canvas().build().unwrap(),
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in 0..n {
            for i in 0..8 {
                yj = (y + j) % 32;
                xi = (x + i) % 64;

                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[yj][xi] == 1 {
                        collision = 1
                    }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }

    pub fn draw_screen(&mut self) {
        if !self.draw_flag {
            return;
        }
        let mut pixel: u8;
        let sc = SCALE as u32;
        // let pt = |&: p: usize| { (p as i16) * (SCALE as i16) };
        let pt = |p: usize| (p as i16) * (SCALE as i16);

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        self.canvas.clear();

        for y in 0..32 {
            for x in 0..64 {
                pixel = if self.gfx[y][x] != 0 { 255 } else { 0 };
                self.canvas.set_draw_color(
                    Color::RGB(pixel, pixel, pixel));
                let _ = self.canvas.fill_rect(
                    Rect::new(
                        pt(x).into(),
                        pt(y).into(),
                        sc,
                        sc
                    )
                );
            }
        }
        
        self.canvas.present();

        self.draw_flag = false;
    }
}
