use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let mut should_stop_loop = false;
    while !should_stop_loop {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Exit;
            match event {
                winit::event::Event::NewEvents(start_cause) => {
                    match start_cause {
                        winit::event::StartCause::Init => {
                            // when we start the window we see that the window got resized automatically
                            // so it got trigerred correctly
                            window.set_inner_size(PhysicalSize {
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
                winit::event::Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            };
        });
    }
}
