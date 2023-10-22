use rlua::Lua;

// Define a Rust struct to hold the configuration parameters
// dERIVE Debug to be able to print the struct
#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u32,
    // Add other parameters as needed
}

pub fn read_lua_config(config_file_path: &str) -> Result<AppConfig, rlua::Error> {
    let lua = Lua::new();

    // Load and execute the Lua configuration file
    lua.context(|ctx| {
        let globals = ctx.globals();

        if let Ok(src) = &std::fs::read_to_string(config_file_path) {
            // Load the Lua file as text and execute it as a chunk
            let config_chunk = ctx.load(src);
            config_chunk.exec()?;
        } else {
            // If the file cannot be read, return an error
            return Err(rlua::Error::RuntimeError(
                "Cannot read Lua configuration file".to_string(),
            ));
        }
        // Extract configuration parameters from Lua global variables
        let host: String = globals.get("host")?;
        let port: u32 = globals.get("port").unwrap();

        // Create and return the Rust configuration struct
        Ok(AppConfig {
            host,
            port,
            // Add other parameters as needed
        })
    })
}
