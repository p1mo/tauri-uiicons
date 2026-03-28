const COMMANDS: &[&str] = &[];
 


fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();  
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
}

