use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::SystemTime;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::{DisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowId};

struct Group {
    window: Rc<Window>,
    context: Context<Rc<Window>>,
    surface: Surface<Rc<Window>, Rc<Window>>,
}

#[derive(Default)]
struct App {
    groups: Vec<Group>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.groups = event_loop
            .available_monitors()
            .map(|m| {
                let window = event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(m))))
                            .with_window_level(winit::window::WindowLevel::AlwaysOnTop),
                    )
                    .unwrap();
                let window = Rc::new(window);

                let context = Context::new(window.clone()).unwrap();

                let surface = Surface::new(&context, window.clone()).unwrap();

                Group {
                    window,
                    context,
                    surface,
                }
            })
            .collect();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                for g in self.groups.iter_mut() {
                    let (width, height) = {
                        let size = g.window.inner_size();
                        (size.width, size.height)
                    };
                    g.surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = g.surface.buffer_mut().unwrap();
                    buffer.fill(0xffffff);
                    buffer.present().unwrap();
                }
            }
            WindowEvent::KeyboardInput { event: asd, .. } => {
                if asd.text == None {
                    return;
                }
                println!("Keyboard event received; stopping {:?}", &event);
                event_loop.exit();
            }
            WindowEvent::MouseInput { .. } => {
                println!("Mouse event received; stopping {:?}", &event);
                event_loop.exit();
            }
            // e => println!("[e] {:?}", e),
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
