use dotenvy::dotenv;

fn main(){
    dotenv().expect("cargo:warning=An error occured in build time");

    for env_var in dotenvy::vars() {
        let (key, value) = env_var;
        println!("cargo:rustc-env={key}={value}");
    }
}