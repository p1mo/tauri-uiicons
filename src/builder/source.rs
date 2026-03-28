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

pub fn url(kind: &str, name: &str) -> Option<String> {
    let source = SOURCES.iter().find(|s| s.0 == kind)?;
    let s = format!("https://raw.githubusercontent.com/{}/refs/heads/{}.svg", source.1, source.2.replace("[NAME]", name));
    Some(s)
}

pub fn http_client() -> anyhow::Result<reqwest::blocking::Client> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(format!("TauriPluginIcons/{}", env!("CARGO_PKG_VERSION")))
        .build()?;
    Ok(client)
}