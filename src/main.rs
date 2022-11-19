use std::{path::Path, fs::File, io::Read};

use mlua::{Lua, Value};


macro_rules! test_speed {
    ($label:literal,$code:block) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️ | {} took {}s",$label, duration.as_secs_f64());
    };
    ($label:literal,$code:stmt) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️ | {} took {}s",$label, duration.as_secs_f64());
    };
}

fn main() {
    let mut code = std::env::args().nth(1).unwrap_or("print('default')".to_string());
    if let Ok(f) =  File::open(code) {
        f.read_to_string(&mut code)
    };
    println!("code : {}", code);

    test_speed!("total", {
        let compiler = mlua::Compiler::new();
        test_speed!("compiler::compile",let bc = compiler.compile(code));

        let lua = mlua::Lua::new();

        test_speed!("lua.load",let chnk = lua.load(&bc));
        test_speed!("chunk.eval",let res : Value = chnk.eval().expect("valid code"));

        println!("Result: {:?}", res);
    });
}