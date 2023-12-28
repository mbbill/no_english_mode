use std::fs;
use std::env;
use std::path::Path;

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

    // Get the directory where the build script is running (OUT_DIR)
    let out_dir = env::var("CARGO_TARGET_DIR").expect("Failed to get CARGO_TARGET_DIR");

    // Define the path to the source bat file
    let source_bat_file = "copy_to_startup.bat";

    // Construct the destination path using the OUT_DIR
    let dest_bat_path = Path::new(&out_dir).join("copy_to_startup.bat");

    // Copy the bat file to the output directory
    fs::copy(source_bat_file, &dest_bat_path).expect("Failed to copy bat file");
}
