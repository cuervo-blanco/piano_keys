use minifb::{Window, Key, MouseMode, MouseButton};
use std::time::{Instant, Duration};

pub struct Button {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    key: Option<Key>,
    color: u32,
    clicked_color: u32,
    last_click_time: Option<Instant>,
    info: Option<String>,
    clicked: bool,
}

impl Button {
    // Constructor 
    pub fn new(
        x: usize, 
        y: usize, 
        width: usize, 
        height: usize, 
        key: Option<Key>,
        color: u32,
        clicked_color: u32,
        info: Option<String>,
    ) -> Self {

        Button {
            x,
            y,
            width,
            height,
            key,
            color,
            clicked_color,
            last_click_time: None,
            info,
            clicked: false,
        }
    }

    // Create a second one that is white on top of the black one
    // Add one that is black on top of the white one

    pub fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize) { 
        let buffer_height = buffer.len() / buffer_width; 
        let color_to_draw = if self.clicked { self.clicked_color } else { self.color };
        for y in self.y..(self.y + self.height) {
            if y >= buffer_height {break;}
            for x in self.x..(self.x + self.width){
                if x >= buffer_width {break;}
                buffer[y * buffer_width + x] = color_to_draw;
            }
        }
    }

    // Check if button is clicked
    pub fn check_click(&mut self, window: &Window) -> bool {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
                let within_x = mouse_x >= self.x as f32 && mouse_x <= (self.x + self.width) as f32;
                let within_y = mouse_y >= self.y as f32 && mouse_y <= (self.y + self.height) as f32;

                if within_x && within_y {
                    self.clicked = true;
                    if let Some(info) = self.get_info() {
                        println!("{}", info);
                    }
                    self.last_click_time = Some(Instant::now());
                    return true; // Button clicked
                }
            }
        } else if let Some(last_time) = self.last_click_time {
            if last_time.elapsed() >= Duration::from_millis(100) {
                self.clicked = false;
                self.last_click_time = None;
            }
        }


        false // Button not clicked
    }

    pub fn get_info(&self) -> Option<String> {
        self.info.clone()
    }

    pub fn get_key(&self) -> Option<Key> {
        self.key.clone()
    }

}
