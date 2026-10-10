use crate::{
    constants::{INITIAL_HEIGHT, INITIAL_WIDTH},
    painting::{Canvas, get_canvas},
    watch::watch_files,
};
use anyhow::Result;
use std::{
    fs,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub fn create_window(html_path: PathBuf, css_path: PathBuf) -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut canvas = rebuild_canvas(INITIAL_WIDTH, INITIAL_HEIGHT, &html_path, &css_path);
    let rx = watch_files(&[html_path.clone(), css_path.clone()]);

    let window = {
        let size = LogicalSize::new(INITIAL_WIDTH as f64, INITIAL_HEIGHT as f64);
        #[allow(deprecated)]
        Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Browser Engine")
                    .with_inner_size(size)
                    .with_min_inner_size(size)
                    .with_resizable(true),
            )?,
        )
    };
    let mut pixels = get_window_pixels(Arc::clone(&window))?;
    let mut input = WinitInputHelper::new();

    write_to_pixels(pixels.frame_mut(), &canvas);

    let mut resize_deadline: Option<Instant> = None;
    let resize_delay = Duration::from_millis(100);
    let mut pending_size: Option<PhysicalSize<u32>> = None;

    #[allow(deprecated)]
    let res = event_loop.run(|event, elwt| match event {
        Event::Resumed => {}
        Event::NewEvents(_) => {
            input.step();
            if rx.try_recv().is_ok() {
                while rx.try_recv().is_ok() {}
                let window_size = window.outer_size();
                canvas =
                    rebuild_canvas(window_size.width, window_size.height, &html_path, &css_path);
                // draining any extra pending events from the save file
                write_to_pixels(pixels.frame_mut(), &canvas);
                window.request_redraw();
            }
        }
        Event::AboutToWait => {
            if let Some(deadline) = resize_deadline {
                if Instant::now() >= deadline {
                    resize_deadline = None;

                    if let Some((width, height)) = pending_size.map(|ps| (ps.width, ps.height)) {
                        pixels
                            .resize_buffer(width, height)
                            .expect("Failed to resize the pixels buffer");

                        canvas = rebuild_canvas(width, height, &html_path, &css_path);
                        write_to_pixels(pixels.frame_mut(), &canvas);

                        window.request_redraw();
                    }
                }
            }

            input.end_step();
        }
        Event::WindowEvent { event, .. } => {
            if input.process_window_event(&event) {
                if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                    elwt.exit();
                    return;
                }
            }

            match event {
                WindowEvent::Resized(size) => {
                    pixels
                        .resize_surface(size.width, size.height)
                        .expect("Failed to resize the pixels surface");

                    resize_deadline = Some(Instant::now() + resize_delay);
                    pending_size = Some(size);

                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render {}", err);
                        elwt.exit();
                        return;
                    }
                }
                _ => (),
            }

            window.request_redraw();
        }
        _ => (),
    })?;

    Ok(res)
}

fn rebuild_canvas(width: u32, height: u32, html_path: &PathBuf, css_path: &PathBuf) -> Canvas {
    let html = read_source(html_path);
    let css = read_source(css_path);

    get_canvas(width, height, html, css)
}

fn read_source(file_name: &PathBuf) -> String {
    fs::read_to_string(file_name).expect("Couldn't read the source")
}

fn get_window_pixels<'win>(window: Arc<Window>) -> Result<Pixels<'win>> {
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        Arc::clone(&window),
    );

    let pixels = Pixels::new(
        window.inner_size().width,
        window.inner_size().height,
        surface_texture,
    )?;

    Ok(pixels)
}

fn write_to_pixels(frame: &mut [u8], canvas: &Canvas) {
    let color_to_slice: Vec<u8> = canvas
        .pixels
        .iter()
        .flat_map(|color| [color.r, color.g, color.b, color.a])
        .collect();

    frame.copy_from_slice(&color_to_slice);
}
