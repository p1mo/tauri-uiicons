mod source;

use source::{ url, http_client };

use std::path::PathBuf;










fn hasher(buffer: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(buffer);
    hasher.finalize().to_hex().to_string()
}

fn serialize(tree: super::types::IconCollection) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(&tree)?)
}

fn deserialize(data: Vec<u8>) -> anyhow::Result<super::types::IconCollection> {
    Ok(bincode::deserialize::<super::types::IconCollection>(&data)?)
}






fn dump_schema(schema_path: PathBuf) -> anyhow::Result<()> {
    if !schema_path.exists() {
        let schema = schemars::schema_for!(super::types::Config);
        let json = serde_json::to_string_pretty(&schema)?;
        std::fs::write(schema_path, json)?;
    }
    Ok(())
}

fn dump_default(config_path: PathBuf) -> anyhow::Result<()> {
    if !config_path.exists() {
        let schema = super::types::Config {
            schema: "gen/schemas/uiicons.json".into(),
            component: super::types::ConfigComponent {
                name: "ui-icon".into(),
                ui_name: "home".into(),
                ui_size: "24px".into(),
                ui_color: "stroke".into(),
            },
            icons: vec![
                super::types::ConfigIcon {
                    source: "feather".into(),
                    icon: "home".into(),
                    name: "home".into()
                }
            ]
        };
        std::fs::write(config_path, serde_json::to_string_pretty(&schema)?)?;
    }
    Ok(())
}

fn load_config(path: PathBuf) -> anyhow::Result<super::types::Config> {
    let mut cfg: super::types::Config = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    cfg.icons.push(super::types::ConfigIcon {
        source: "tabler-outline".into(),
        icon:   "error-404".into(),
        name:   "internal-error-404".into()
    });
    Ok(cfg)
}





fn load_from_disk(bin: PathBuf) -> anyhow::Result<super::types::IconCollection> {
    let mut data = super::types::IconCollection::new();
    if bin.exists() {
        data = deserialize(std::fs::read(bin)?)?;
    }
    Ok(data)
}

fn save_to_disk(sum: PathBuf, bin: PathBuf, tree: super::types::IconCollection) -> anyhow::Result<()> {

    let data = serialize(tree)?;
    
    if !sum.exists() && !bin.exists() {
        std::fs::write(sum, hasher(&data))?;
        std::fs::write(bin, data)?;
        return Ok(());
    }
    
    if sum.exists() && !bin.exists() {
        std::fs::write(sum, hasher(&data))?;
        std::fs::write(bin, data)?;
        return Ok(());
    }
    
    if sum.exists() && bin.exists() {
        let hash = hasher(&data);
        if std::fs::read_to_string(&sum)? != hash {
            std::fs::write(sum, hash)?;
            std::fs::write(bin, data)?;
            return Ok(());
        }
    } 

    Ok(())
}

















fn build_icons() -> anyhow::Result<String, anyhow::Error> { // CARGO_MANIFEST_DIR

    let odir = PathBuf::from(std::env::var("OUT_DIR")?);
    let rdir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let gen_dir = rdir.join("gen");
    if !gen_dir.exists() {
        std::fs::create_dir_all(&gen_dir)?;
    }

    let bin_path    = gen_dir.join("uiicons.bin");
    let check_dir   = odir.join("uiicons.checksum");
    let schema_path = gen_dir.join("schemas").join("uiicons.json");
    let config_path = rdir.join("tauri-plugin-icons.json");

    let mut instance = load_from_disk(bin_path.clone())?;

    dump_schema(schema_path)?;
    dump_default(config_path.clone())?;

    let client = http_client()?;
    let config = load_config(config_path.clone())?;

    for item in &config.icons {
        if !instance.contains_key(&item.name) {
            if let Some(uri) = url(&item.source, &item.icon) {
                let bytes = client.get(uri).send()?.bytes()?;
                instance.insert(
                    item.name.clone(), 
                    (
                        item.source.clone(),
                        item.icon.clone(),
                        bytes.to_vec()
                    )
                );
            } else {
                panic!("not found: {}, {}", &item.source, &item.icon);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    instance.retain(
        |key, item| {
            config.icons
                .iter()
                .find(|s| { s.name == *key && s.source == item.0 && s.icon == item.1 })
                .map(|f| f.name.clone()) == Some(key.to_string())
        }
    );

    save_to_disk(check_dir, bin_path, instance)?;

    Ok(config_path.display().to_string())

}






pub fn build() {
    let path = match build_icons() {
        Err(e) => panic!("{}", e.to_string()),
        Ok(str) => str,
    };
    println!("cargo:warn={}", path);
    println!("cargo:rerun-if-changed={}", path);
}