use crate::game_boy::components::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game_boy::GameBoy;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const GAME_BOY_FPS: f64 = 59.7;
const WINDOW_SCALE_FACTOR: u32 = 3;

pub fn run(game_boy: &mut GameBoy) {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(
            SCREEN_WIDTH as f64 * WINDOW_SCALE_FACTOR as f64,
            SCREEN_HEIGHT as f64 * WINDOW_SCALE_FACTOR as f64,
        );
        WindowBuilder::new()
            .with_title("LemonGB")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Failed to create window")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
            .expect("Failed to create pixel buffer")
    };

    const FRAME_DURATION: Duration = Duration::from_nanos((1_000_000_000.0 / GAME_BOY_FPS) as u64);

    let res = event_loop.run(|event, elwt| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let frame = pixels.frame_mut();
            frame.copy_from_slice(game_boy.get_frame_buffer());

            if let Err(err) = pixels.render() {
                error!("pixels.render error: {}", err);
                elwt.exit();
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface error: {}", err);
                    elwt.exit();
                    return;
                }
            }

            let frame_start = Instant::now();

            game_boy.finish_frame();
            let elapsed = frame_start.elapsed();

            if elapsed < FRAME_DURATION {
                sleep(FRAME_DURATION - elapsed);
            }

            window.request_redraw();
        }
    });
}
