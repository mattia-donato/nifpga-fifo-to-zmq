fn main(){
    println!("cargo:rustc-link-lib=dylib=nifpga");
    println!("cargo:rustc-link-search=native=./");
}

