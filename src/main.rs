use macroquad::prelude::*;

#[macroquad::main("Rusteroids")] // Bar Title
async fn main() {
    loop {
        // Main render loop
        clear_background(GRAY);

        next_frame().await;
    }
}
