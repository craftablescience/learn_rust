// todo: hide winit behind wrappers
pub use winit;
pub use winit::{
    event,
    window::Window,
};

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct Application {
    #[allow(dead_code)] //todo(web)
    launch_arguments: Vec<String>,
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl Application {
    pub fn new(title: &str) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
            } else {
                env_logger::init();
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
            let launch_arguments = std::env::args().collect();
        #[cfg(target_arch = "wasm32")]
            let launch_arguments = vec![]; //todo

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        //todo: will eventually need to generate index.html
        #[cfg(target_arch = "wasm32")]
        {
            // winit prevents sizing with CSS, so we have to set the size manually on web
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450 as i16, 400 as i16));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    // set title
                    doc.set_title(title);

                    // add canvas
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body!");
        }

        Self {
            launch_arguments,
            event_loop,
            window,
        }
    }

    #[allow(unused_variables)]
    pub fn has_argument(&self, arg: &str) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.launch_arguments.iter().any(|str| str == arg)
        }
        #[cfg(target_arch = "wasm32")]
        {
            false //todo(web)
        }
    }
}
