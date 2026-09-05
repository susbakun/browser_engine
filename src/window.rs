use crate::{
    constants::{HEIGHT, WIDTH},
    painting::{Canvas, get_canvas},
    watch::watch_files,
};
use anyhow::Result;
use std::{fs, path::PathBuf, sync::Arc};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub fn create_window(html_path: PathBuf, css_path: PathBuf) -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut canvas = rebuild_canvas(&html_path, &css_path);
    let rx = watch_files(&[html_path.clone(), css_path.clone()]);

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
    let mut pixels = get_window_pixels(Arc::clone(&window))?;
    let mut input = WinitInputHelper::new();

    write_to_pixels(pixels.frame_mut(), &canvas);

    #[allow(deprecated)]
    let res = event_loop.run(|event, elwt| match event {
        Event::Resumed => {}
        Event::NewEvents(_) => {
            input.step();
            if rx.try_recv().is_ok() {
                while rx.try_recv().is_ok() {}
                canvas = rebuild_canvas(&html_path, &css_path);
                write_to_pixels(pixels.frame_mut(), &canvas);
                window.request_redraw();
            }
        }
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

fn rebuild_canvas(html_path: &PathBuf, css_path: &PathBuf) -> Canvas {
    let html = read_source(html_path);
    let css = read_source(css_path);

    get_canvas(html, css)
}

fn read_source(file_name: &PathBuf) -> String {
    fs::read_to_string(file_name).expect("Couldn't read the source")
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
