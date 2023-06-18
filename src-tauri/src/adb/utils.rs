macro_rules! run_adb_command {
    ($param:expr, $args:expr) => {{
        use tauri::api::process::Command;

        #[cfg(debug_assertions)]
        println!("Running adb {} {}", $param.join(" "), $args.join(" "));

        let result = Command::new("adb")
            .args($param)
            .args($args)
            .output()
            .expect("error while running command")
            .stdout;

        #[cfg(debug_assertions)]
        println!("Result: {}", String::from_utf8_lossy(&result));

        result
    }};
    ($args:expr) => {
        run_adb_command!([""], $args)
    };
}
pub(crate) use run_adb_command;

macro_rules! skip_until {
    ($iterator:expr, $search:expr) => {{
        let mut lines = $iterator.skip_while(|line| !line.starts_with($search));
        lines.next();
        lines
    }};
}
pub(crate) use skip_until;
