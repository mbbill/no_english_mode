

fn main() {

    // use std::process::Command;
    // // Get the path to the output directory
    // let out_dir = env::var("OUT_DIR").unwrap();

    // // Define paths for the .rc and .res files
    // let rc_path = "assets/app.rc";
    // let res_path = Path::new(&out_dir).join("app.res");

    // // Compile the resource file
    // Command::new("rc")
    //     .args(&["/fo", res_path.to_str().unwrap(), rc_path])
    //     .status()
    //     .unwrap();

    // // Tell cargo to link the resource file
    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-arg=app.res");


    println!("cargo:rustc-link-arg=assets/app.res");
}
