/// Sources
/// 
/// `https://raw.githubusercontent.com`
/// 
/// | IconSet           | Website                           | Github                                            | raw path                                                  |
/// |:------------------|:----------------------------------|:--------------------------------------------------|:----------------------------------------------------------|
/// | tabler            | https://tabler.io                 | https://github.com/tabler/tabler-icons            | `refs/heads/main/icons/outline/a-b-2.svg`                 |
/// | simple-icons      | https://simpleicons.org           | https://github.com/simple-icons/simple-icons      | `refs/heads/develop/icons/1001tracklists.svg`             |
/// | feather           | https://feathericons.com/         | https://github.com/feathericons/feather           | `refs/heads/main/icons/activity.svg`                      |
/// | material-design   | https://fonts.google.com/icons    | https://github.com/google/material-design-icons   | `refs/heads/master/src/action/123/materialicons/24px.svg` |
/// | lucide            | https://lucide.dev                | https://github.com/lucide-icons/lucide            | `refs/heads/main/icons/a-arrow-down.svg`                  |
/// | heroicons         | https://heroicons.com             | https://github.com/tailwindlabs/heroicons         | `refs/heads/master/src/24/outline/academic-cap.svg`       |
/// | bootstrap         | https://icons.getbootstrap.com    | https://github.com/twbs/icons                     | `refs/heads/main/icons/0-circle-fill.svg`                 |
/// | remixicon         | https://remixicon.com             | https://github.com/Remix-Design/RemixIcon         | `refs/heads/master/icons/Arrows/arrow-down-box-fill.svg`  |
/// | iconoir           | https://iconoir.com               | https://github.com/iconoir-icons/iconoir          | `refs/heads/main/icons/regular/accessibility-sign.svg`    |
/// | phosphor          | https://phosphoricons.com         | https://github.com/phosphor-icons/core            | `refs/heads/main/assets/bold/acorn-bold.svg`              |
/// | thesvg            | https://thesvg.org                | https://github.com/glincker/thesvg                | `refs/heads/main/public/icons/1and1/default.svg`          |
/// | devicons          | https://devicon.dev               | https://github.com/devicons/devicon               | `refs/heads/master/icons/aarch64/aarch64-plain.svg`       |
/// 
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

    ("lucide",          "lucide-icons/lucide",          "main/icons/[NAME]"),

    ("heroicons",       "tailwindlabs/heroicons",       "master/src/24/outline/[NAME]"),
    ("heroicons-solid", "tailwindlabs/heroicons",       "master/src/24/solid/[NAME]"),

    ("bootstrap",       "twbs/icons",                   "main/icons/[NAME]"),

    ("remixicon",       "Remix-Design/RemixIcon",       "master/icons/[NAME]"),

    ("iconoir",         "iconoir-icons/iconoir",        "main/icons/regular/[NAME]"),
    ("iconoir-solid",   "iconoir-icons/iconoir",        "main/icons/solid/[NAME]"),

    ("phosphor-thin",       "phosphor-icons/core",          "main/assets/thin/[NAME]"),
    ("phosphor-light",      "phosphor-icons/core",          "main/assets/light/[NAME]"),
    ("phosphor",            "phosphor-icons/core",          "main/assets/regular/[NAME]"),
    ("phosphor-bold",       "phosphor-icons/core",          "main/assets/bold/[NAME]"),
    ("phosphor-fill",       "phosphor-icons/core",          "main/assets/fill/[NAME]"),
    ("phosphor-duotone",    "phosphor-icons/core",          "main/assets/duotone/[NAME]"),

    ("thesvg",              "phosphor-icons/core",          "main/assets/thin/[NAME]/default"),
    ("thesvg-mono",         "phosphor-icons/core",          "main/assets/thin/[NAME]/mono"),

    ("devicons",            "devicons/devicon",             "master/icons/[NAME]/[NAME]-original"),
    ("devicons-plain",      "devicons/devicon",             "master/icons/[NAME]/[NAME]-plain"),
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