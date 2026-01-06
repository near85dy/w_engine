use glfw::{PWindow, Context, fail_on_errors, Glfw};

pub struct GLFW {
    glfw: Glfw,
    window: PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl GLFW {
    pub fn new() -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw
            .create_window(1920, 1080, "W Engine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_all_polling(true);

        glfw.set_swap_interval(glfw::SwapInterval::None);

        Self { glfw, window, events }
    }

    pub fn update(&mut self)
    {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn is_open(&self) -> bool {
        return !self.window.should_close();
    }

    pub fn get_window_instance(&mut self) -> &mut PWindow
    {
        return &mut self.window;
    }
}