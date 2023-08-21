use macroquad::prelude::*;

#[macroquad::main("Rusteroids")] // Bar Title
async fn main() {
    loop {
        // Input and other events <-- here
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // Simulate here

        // Main render loop
        clear_background(GRAY);

        next_frame().await;
    }
}
