// main.rs

#[cfg(feature = "lua")]
mod lua {
    use rhai::{Engine, EvalAltResult};

    pub fn run_lua_script() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();

        let lua_script = r#"
           print("Hello from Lua!")
        "#;

        engine.eval::<()>(lua_script)?;

        Ok(())
    }
}

#[cfg(not(feature = "lua"))]
mod lua {
    pub fn run_lua_script() -> Result<(), ()> {
        println!("Lua support is not enabled.");
        Ok(())
    }
}

fn main() {
    println!("Hello from Rust!");

    if let Err(err) = lua::run_lua_script() {
        eprintln!("Error running Lua script: {:?}", err);
    }
}
