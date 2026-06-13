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

<div align="center">

| Icon Set          | Name Example       | Website                                              | Github                                          |
| :---------------- | :----------------- | :--------------------------------------------------- | :---------------------------------------------- |
| tabler-filled     | user               | https://tabler.io/icons                              | https://github.com/tabler/tabler-icons          |
| tabler-outline    | user               | https://tabler.io/icons                              | https://github.com/tabler/tabler-icons          |
| simple-icons      | debian             | https://simpleicons.org                              | https://github.com/simple-icons/simple-icons    |
| feather           | activity           | https://feathericons.com                             | https://github.com/feathericons/feather         |
| md                | action/123         | https://fonts.google.com/icons                       | https://github.com/google/material-design-icons |
| md-outline        | action/accessible  | https://fonts.google.com/icons?icon.style=Outlined   | https://github.com/google/material-design-icons |
| md-round          | communication/call | https://fonts.google.com/icons?icon.style=Rounded    | https://github.com/google/material-design-icons |
| md-sharp          | navigation/apps    | https://fonts.google.com/icons?icon.style=Sharp      | https://github.com/google/material-design-icons |

</div>


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


#### `tauri-plugin-icons.json`
```json
    {
        "$schema": "gen/schemas/uiicons.json",
        "component": {
            "name": "ui-icon",
            "uiName": "home",
            "uiSize": "24px",
            "uiColor": "stroke"
        },
        "icons": [ 

            {
                "source": "ICON_SET",
                "icon"  : "REAL_ICON_NAME",
                "name"  : "CUSTOM_ICON_NAME"
            },

            {
            "source": "tabler-outline",
            "icon"  : "user",
            "name"  : "home_outline"
            },
            {
            "source": "simple-icons",
            "icon"  : "debian",
            "name"  : "brand_debian"
            },
            {
            "source": "feather",
            "icon"  : "activity",
            "name"  : "activity"
            },
            {
            "source": "md-sharp",
            "icon"  : "action/accessible",
            "name"  : "act_accessible"
            }
        ]
    }
```


#### `index.html`
```html
    <ui-icon name="home_outline"></ui-icon>
    <ui-icon name="brand_debian"></ui-icon>
    <ui-icon name="activity"></ui-icon>
    <ui-icon name="act_accessible"></ui-icon>
```

---

## Auto Generated

#### `src/tauri-plugin-icons.json`
>this file is auto-generated (once for you) and has a schema. you can edit it and add icons

#### `gen/schemas/uiicons.json`
>is the schema file

#### `gen/uiicons.bin`
>holds the icons