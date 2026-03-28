use std::path::PathBuf;
use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use blake3::Hasher;

fn hasher(buffer: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(buffer);
    hasher.finalize().to_hex().to_string()
}






#[derive(Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct Config {
    #[serde(rename = "$schema")]
    schema : String,

    #[serde(rename = "componentName")]
    component_name : String,

    icons : Vec<ConfigIcon>,
}

#[derive(Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ConfigIcon {
    /// source of icons like "tabler-filled", "feather"
    source : String,
    /// icon name from source like "home", "settings"
    icon : String,
    /// frontend icon name "home_new", "settings_new"
    name : String,
}


impl Config {


    fn schema_path(&self, path: PathBuf) -> PathBuf {
        path.join("gen").join("schemas").join("uiicons.json")
    }

    fn config_path(&self, path: PathBuf) -> PathBuf {
        path.join("tauri-plugin-icons.json")
    }

    fn dump_schema(&self, path: PathBuf) -> anyhow::Result<()> {
        let path = self.schema_path(path);
        if !path.exists() {
            let schema = schemars::schema_for!(Config);
            let json = serde_json::to_string_pretty(&schema)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }

    fn dump_default(&self, path: PathBuf) -> anyhow::Result<()> {
        let path = self.config_path(path);
        if !path.exists() {
            let schema = Config {
                schema: "gen/schemas/uiicons.json".into(),
                component_name: "ui-icons".into(),
                icons: vec![
                    ConfigIcon {
                        source: "feather".into(),
                        icon: "home".into(),
                        name: "home".into()
                    }
                ]
            };
            std::fs::write(path, serde_json::to_string_pretty(&schema)?)?;
        }
        Ok(())
    }

    fn load(path: PathBuf) -> anyhow::Result<Self> {
        let mut this = Self::default();
        this.dump_schema(path.clone())?;
        this.dump_default(path.clone())?;
        this = serde_json::from_str(&std::fs::read_to_string(this.config_path(path))?)?;
        Ok(this)
    }

    fn add_default(&mut self) -> anyhow::Result<()> {
        if self.icons.iter().find(|s| {
            s.source == "tabler-outline" && s.icon == "error-404" && s.name == "internal-error-404"
        }).is_none() {
            self.icons.push(ConfigIcon {
                source: "tabler-outline".into(),
                icon:   "error-404".into(),
                name:   "internal-error-404".into()
            });
        }
        Ok(())
    }


}









#[derive(serde::Deserialize, serde::Serialize)]
struct Icons(pub super::IconCollection, Vec<ConfigIcon>, String);

impl Icons {

    fn new(icons: Vec<ConfigIcon>) -> Self {
        Self(super::IconCollection::new(), icons, String::new())
    }

    pub fn load_fs(path: PathBuf, icons: Vec<ConfigIcon>) -> anyhow::Result<Self> {
        let mut this = Self::new(icons);
        if !path.exists() {
            let encoded = bincode::serialize(&this.0).unwrap();
            this.2 = hasher(&encoded);
            std::fs::write(&path, encoded)?;
        }
        if path.exists() {
            let data = std::fs::read(&path)?;
            this.2 = hasher(&data);
            let collection: super::IconCollection = bincode::deserialize(&data).unwrap();
            this.0 = collection;
        }
        Ok(this)
    }

    fn compare(&mut self) {
        self.0.retain(
            |key, item| {
                self.1
                    .iter()
                    .find(|s| { s.name == *key && s.source == item.source && s.icon == item.icon })
                    .map(|f| f.name.clone()) == Some(key.to_string())
            }
        );
    }

    fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn download(&mut self) -> anyhow::Result<()> {
        let client = http_client()?;
        for item in &self.1 {
            if !self.contains(&item.name) {
                if let Some(uri) = url(&item.source, &item.icon) {
                    let bytes = client.get(uri).send()?.bytes()?;
                    self.0.insert(item.name.clone(), super::SvgFile {
                        data: bytes.to_vec(),
                        source: item.source.clone(),
                        icon: item.icon.clone()
                    });
                } else {
                    panic!("not found: {}, {}", &item.source, &item.icon);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok(())
    }

    pub fn save_fs(&self, path: PathBuf, out: PathBuf) -> anyhow::Result<()> {
        let out_dir = out.join("uiicons.checksum");
        if !out_dir.exists() && !path.exists() {
            std::fs::write(&out_dir, self.2.clone())?;
            std::fs::write(&path, bincode::serialize(&self.0)?)?;
        }

        if out_dir.exists() && path.exists() {
            let sum = std::fs::read_to_string(&out_dir)?;
            if sum != self.2 {
                std::fs::write(out_dir, self.2.clone())?;
                std::fs::write(path, bincode::serialize(&self.0)?)?;
            }
        }

        Ok(())
    }

}













/// Sources
/// 
/// `https://raw.githubusercontent.com`
/// 
/// + tabler:         https://github.com/tabler/tabler-icons            | `refs/heads/main/icons/outline/a-b-2.svg`
/// + simple-icons:   https://github.com/simple-icons/simple-icons      | `refs/heads/develop/icons/1001tracklists.svg`
/// + feather:        https://github.com/feathericons/feather           | `refs/heads/main/icons/activity.svg`
/// + material-design:https://github.com/google/material-design-icons   | `refs/heads/master/src/action/123/materialicons/24px.svg`
/// 
static SOURCES: &[(&str, &str, &str)] = &[
    ("tabler-filled",   "tabler/tabler-icons",          "main/icons/filled/[NAME]"),
    ("tabler-outline",  "tabler/tabler-icons",          "main/icons/outline/[NAME]"),
    ("simple-icons",    "simple-icons/simple-icons",    "develop/icons/[NAME]"),
    ("feather",         "feathericons/feather",         "main/icons/[NAME]"),
    ("md",              "google/material-design-icons", "master/src/[NAME]/materialicons/24px"),
    ("md-outline",      "google/material-design-icons", "master/src/[NAME]/materialiconsoutlined/24px"),
    ("md-round",        "google/material-design-icons", "master/src/[NAME]/materialiconsround/24px"),
    ("md-sharp",        "google/material-design-icons", "master/src/[NAME]/materialiconssharp/24px"),
];

fn url(kind: &str, name: &str) -> Option<String> {
    let source = SOURCES.iter().find(|s| s.0 == kind)?;
    let s = format!("https://raw.githubusercontent.com/{}/refs/heads/{}.svg", source.1, source.2.replace("[NAME]", name));
    Some(s)
}

fn http_client() -> anyhow::Result<Client> {
    let client = ClientBuilder::new()
        .user_agent(format!("TauriPluginIcons/{}", env!("CARGO_PKG_VERSION")))
        .build()?;
    Ok(client)
}










fn build_icons() -> anyhow::Result<(), anyhow::Error> { // CARGO_MANIFEST_DIR

    let odir = PathBuf::from(std::env::var("OUT_DIR")?);
    let rdir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let gen_dir = root_dir.join("gen");
    if !shared.exists() {
        std::fs::create_dir_all(&shared)?;
    }

    let bin_path    = gen_dir.join("uiicons.bin");
    let scheme_path = gen_dir.join("schemas").join("uiicons.json");
    let config_path = rdir.join("tauri-plugin-icons.json");








    

    Ok(())

}






pub fn build() {
    match build_icons() {
        Err(e) => panic!("{}", e.to_string()),
        _ => {}
    }
}