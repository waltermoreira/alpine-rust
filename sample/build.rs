fn main() {
    #[cfg(target_os = "linux")]
    {
        // List here all the native libraries that need to be added to the
        // binary. Make sure to add the corresponding package to the
        // Dockerfile (namely, the Alpine package that contains the .a files).
        println!("cargo:rustc-link-lib=static=zmq");
        println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static=sodium");
    }
}
