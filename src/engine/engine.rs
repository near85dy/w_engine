use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use crate::engine::core::core::Core;
use crate::engine::renderer::render_command::RenderCommand;
use crate::engine::renderer::{Renderer};
use crate::engine::glfw::GLFW;
use crate::engine::resource_manager::model_loader::ModelLoader;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self { Self {} }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel::<Vec<RenderCommand>>();

        let (tx_done, rx_done) = mpsc::channel::<()>();

        let render_thread = thread::spawn(move || {
            let mut glfw = GLFW::new();
            let mut renderer = Renderer::new(glfw.get_window_instance());
            renderer.initialize();

            loop {
                let commands = match rx.recv() {
                    Ok(cmds) => cmds,
                    Err(_) => break,
                };

                renderer.draw(commands);

                glfw.update();
                
                if !glfw.is_open() {
                    break;
                }

                if tx_done.send(()).is_err() {
                    break;
                }
            }
        });


        // ------------- Main thread logic ------------

        let mut core = Core::new();

        let mut render_commands: Vec<RenderCommand> = Vec::new();

        loop {
            core.update(&mut render_commands);

            tx.send(render_commands).unwrap();
            
            rx_done.recv().unwrap();
                
            render_commands = Vec::new();
        }
        let _ = render_thread.join();
    }
}
