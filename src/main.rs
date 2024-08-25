use minifb::{Key, Window, WindowOptions};
use guitools::piano::{white_keys, black_keys};
use guitools::button::Button;

fn main() {

    let init_width: usize = 800;
    let init_height: usize = 600;

    let mut buffer: Vec<u32> = vec![0; init_width * init_height];


    let mut window = match Window::new(
        "Piano", init_width, init_height, 
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        }) {
        Ok(window) => window,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    let (new_width, new_height) = window.get_size();
    let mut width = new_width;
    let mut height = new_height;
    buffer.resize(width * height, 0);

    let mut keys = initialize_keys(width, height);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (new_width, new_height) = window.get_size();

        if new_width != width || new_height != height {
            width = new_width;
            height = new_height;
            buffer.resize(width * height, 0);
            keys = initialize_keys(width, height);
        }

        // Clear the buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0x333333; // Grey
        }

        // Handle button interactions and drawings
        for key in &mut keys {
            key.check_click(&window);
            key.draw(&mut buffer, width);
        };


        window.update_with_buffer(&buffer, width, height).unwrap();
    }

}
fn initialize_keys(width: usize, height: usize) -> Vec<Button> {
    let key_width = 150;
    let total_keys = width / key_width;
    let total_black_keys = (total_keys as f32 * 0.71428571) as usize;
    let key_height: usize = height / 3;

    let white_keys: Vec<Button> = white_keys(
        total_keys, 
        key_width, 
        height, 
        key_height
    );
    let black_keys: Vec<Button> = black_keys(
        total_black_keys, 
        key_width, 
        height, 
        key_height
    );

    let mut keys: Vec<Button> = Vec::new();
    keys.extend(white_keys);
    keys.extend(black_keys);

    keys
}
