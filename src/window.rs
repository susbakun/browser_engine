use crate::{
    constants::{HEIGHT, WIDTH},
    painting::Canvas,
};
use anyhow::Result;
use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub fn create_window(canvas: &Canvas) -> Result<()> {
    let event_loop = EventLoop::new()?;

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        #[allow(deprecated)]
        Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Browser Engine")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )?,
        )
    };
    let mut input = WinitInputHelper::new();

    let mut pixels = get_window_pixels(Arc::clone(&window))?;
    let frame = pixels.frame_mut();

    write_to_pixels(frame, canvas);

    #[allow(deprecated)]
    let res = event_loop.run(|event, elwt| match event {
        Event::Resumed => {}
        Event::NewEvents(_) => input.step(),
        Event::AboutToWait => input.end_step(),
        Event::WindowEvent { event, .. } => {
            if input.process_window_event(&event) {
                if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                    elwt.exit();
                    return;
                }
            }

            if event == WindowEvent::RedrawRequested {
                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render {}", err);
                    elwt.exit();
                    return;
                }
            }

            window.request_redraw();
        }
        _ => (),
    })?;

    Ok(res)
}

fn get_window_pixels<'win>(window: Arc<Window>) -> Result<Pixels<'win>> {
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    Ok(pixels)
}

fn write_to_pixels(frame: &mut [u8], canvas: &Canvas) {
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let src = canvas.pixels[y * WIDTH as usize + x];
            let dst = (y * WIDTH as usize + x) * 4;
            frame[dst] = src.r;
            frame[dst + 1] = src.g;
            frame[dst + 2] = src.b;
            frame[dst + 3] = src.a;
        }
    }
}
