use std::cell::RefCell;
use winit::{
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
            // if we use different ControlFlow it seems to work but it lock the thread
            // thats not what i want here. `ControlFlow::Exit` is important as we can poll event
            // without locking the thread
            *control_flow = ControlFlow::Exit;
            match event {
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
                winit::event::Event::RedrawRequested(_) => {}
                _ => (),
            };
        });
    }
}
