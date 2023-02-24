fn main() {
    println!("cargo:rustc-link-lib=dylib=EGL");

    // if the os is windows, say "why the fuck are you using windows"
    if cfg!(windows) {
        println!("why the fuck are you using windows");
    }
}
