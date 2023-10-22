use lua_interface::from_lua;
fn main() {
    let lua_code = r#"
      Config = {
        rootdir = "/abc/123",
        debug = true,
      }
  "#;

    match from_lua(lua_code) {
        Ok(rust_config) => {
            // Use the Rust Config struct
            println!("{:?}", rust_config);
        }
        Err(err) => {
            eprintln!("Error parsing Lua config: {:?}", err);
        }
    }
}
