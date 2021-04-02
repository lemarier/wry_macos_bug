use std::{cell::RefCell, time::Instant};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};

use wry::webview::WebViewBuilder;

fn main() {
    let mut event_loop = EventLoop::new();

    let webview = WebViewBuilder::new(Window::new(&event_loop).expect("Unable to create window"))
        .unwrap()
        .load_url("https://tauri.studio")
        .expect("Unable to load webview url")
        .build()
        .expect("Unable to create webview");

    let webview_cell = RefCell::new(webview);

    let mut should_stop_loop = false;
    while !should_stop_loop {
        event_loop.run_return(|event, _, control_flow| {
            let webview = webview_cell.borrow();
            //webview.fullscreen();
            // if we use different ControlFlow it seems to work but it lock the thread
            // thats not what i want here. `ControlFlow::Exit` is important as we can poll event
            // without locking the thread
            *control_flow = ControlFlow::WaitUntil(Instant::now());

            match event {
                winit::event::Event::NewEvents(start_cause) => {
                    match start_cause {
                        winit::event::StartCause::Init => {
                            // window got resized only when the webview got rendered
                            // looks like the webview is blocking something here
                            // if we start resizing the window, it got unblocked and window
                            // got resized to 1024 x 768
                            std::thread::sleep(std::time::Duration::from_millis(1000));
                            webview.window().set_inner_size(PhysicalSize {
                                width: 1024,
                                height: 768,
                            });
                        }
                        _ => {}
                    }
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    should_stop_loop = true;
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(_),
                    ..
                } => {
                    webview.resize().unwrap();
                }
                winit::event::Event::MainEventsCleared => {
                    webview.window().request_redraw();
                }
                _ => {
                    *control_flow = ControlFlow::Exit;
                }
            };
        });
    }
}
