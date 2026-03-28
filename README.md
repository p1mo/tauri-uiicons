<center><h1>tauri-uiicons</h1></center>

<center class="display: flex; align-items: center; justify-content: center; width: 100%; gap: 5px;">
<h3>SVG Icons as HTMLElement</h3>
</center>

<center class="display: flex; align-items: center; justify-content: center; width: 100%; gap: 5px;">
    <span style="display: inline-block;">
        <a href="https://crates.io/crates/tauri-uiicons">
            <img src="https://img.shields.io/crates/v/tauri-uiicons?style=flat-square">
        </a>
    </span>
</center>


#### `Cargo.toml`
```toml
    [dependencies]
    tauri-uiicons = { version = "0.1.0", features = [ "build" ] }

    [dependencies]
    tauri-uiicons = "0.1.0"
```


#### `main.rs`
```rust
    .plugin(tauri_uiicons::init(include_bytes!("../gen/uiicons.bin")))
```


#### `build.rs`
```rust
    fn main() {
        tauri_uiicons::build(); 
        tauri_build::build()
    }
```

---

#### `src/tauri-plugin-icons.json`
>this file is auto-generated (once for you) and has a schema. you can edit it and add icons

#### `gen/schemas/uiicons.json`
>is the schema file

#### `gen/uiicons.bin`
>holds the icons