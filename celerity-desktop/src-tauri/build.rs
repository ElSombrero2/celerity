fn main() {
    tauri_build::build();
    dotenvy::from_filename(".env.desktop").expect("Error while loading env");

    for env_var in dotenvy::vars() {
        let (key, value) = env_var;
        println!("cargo:rustc-env={key}={value}");
    }
}
