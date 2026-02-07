use std::{fs, path::Path, ptr::null};

use mlua::{Function, Table};

pub struct Lua {
    pub lua: mlua::Lua,
    pub globals: Option<Table>,
}

impl Lua {
    pub fn new() -> Self {
        Self { 
            lua: mlua::Lua::new(),
            globals: None,
        }
    }

    pub fn initialize(&mut self) -> mlua::Result<()> {
        self.globals = Some(self.lua.globals());
        let globals: &Table = self.globals.as_ref().unwrap();

        let path = Path::new("./assets/lua/main.lua");

        let code = fs::read_to_string(path)?;

        if let Err(e) = self.lua.load(&code).exec() {
            println!("Lua Error: {}", e);  
        }

        let start_fn: Option<Function> = globals.get("start")?;
        if let Some(start) = &start_fn {
            let _ : () = start.call(())?; 
        }

        Ok(())
    }

    pub fn update(&self) -> mlua::Result<()> {
        let globals: &Table = self.globals.as_ref().unwrap();
        let update_fn: Option<Function> = globals.get("update")?;
        if let Some(update) = &update_fn {
            let _ : () = update.call(())?;
        }

        Ok(())
    }
}