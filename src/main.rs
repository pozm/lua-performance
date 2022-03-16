use rluau::{
    compiler::{self, CompileError, CompileOptions},
    vm
};

macro_rules! test_speed {
    ($label:literal,$code:block) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️{} took {}s",$label, duration.as_secs_f64());
    };
    ($label:literal,$code:stmt) => {
        let start = std::time::Instant::now();
        $code;
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("⏱️{} took {}s",$label, duration.as_secs_f64());
    };
}

fn main() -> Result<(), CompileError> {
    let mut opts = CompileOptions::default();
    let code = std::env::args().nth(1).unwrap_or("print('default')".to_string());
    println!("code : {}", code);

    test_speed!("total", {
        test_speed!("compiler::compile",let bc = compiler::compile(code, &mut opts)?);

        let lua = vm::Luau::new();

        test_speed!("lua.load",lua.load("main", bc, None).expect("Failed to load bytecode"));
        test_speed!("lua.pcall",let mut res = lua.pcall(0, 0, 0));

        println!("Result: {:?}", res);
    });
    Ok(())
}