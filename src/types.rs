pub type IconCollection = std::collections::BTreeMap<String, (String, String, Vec<u8>)>;



#[derive(Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Config {
    #[serde(rename = "$schema")]
    /// Default schema for this Config
    pub schema : String,
    /// Component settings for Javascript Element
    pub component : ConfigComponent,
    /// Icons to Download and Embed into the app
    /// 
    /// | IconSet           | Website                           | Github                                            |
    /// |:------------------|:----------------------------------|:--------------------------------------------------|
    /// | tabler            | https://tabler.io                 | https://github.com/tabler/tabler-icons            |
    /// | simple-icons      | https://simpleicons.org           | https://github.com/simple-icons/simple-icons      |
    /// | feather           | https://feathericons.com/         | https://github.com/feathericons/feather           |
    /// | material-design   | https://fonts.google.com/icons    | https://github.com/google/material-design-icons   |
    /// | lucide            | https://lucide.dev                | https://github.com/lucide-icons/lucide            |
    /// | heroicons         | https://heroicons.com             | https://github.com/tailwindlabs/heroicons         |
    /// | bootstrap         | https://icons.getbootstrap.com    | https://github.com/twbs/icons                     |
    /// | remixicon         | https://remixicon.com             | https://github.com/Remix-Design/RemixIcon         |
    /// | iconoir           | https://iconoir.com               | https://github.com/iconoir-icons/iconoir          |
    /// | phosphor          | https://phosphoricons.com         | https://github.com/phosphor-icons/core            |
    /// | thesvg            | https://thesvg.org                | https://github.com/glincker/thesvg                |
    /// | devicons          | https://devicon.dev               | https://github.com/devicons/devicon               |
    ///
    pub icons : Vec<ConfigIcon>,
}
 
#[derive(Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ConfigIcon {
    /// #### source of icons like "tabler-filled", "feather"
    /// 
    /// + `tabler-filled` | `tabler-outline`
    /// + `simple-icons`
    /// + `feather`
    /// + `md` | `md-outline` | `md-round` | `md-sharp`
    pub source : String,
    /// #### icon name from source like "home", "settings"
    /// 
    /// + tabler:           https://github.com/tabler/tabler-icons          | `home`, `settings`
    /// + simple-icons:     https://github.com/simple-icons/simple-icons    | `android`, `1and1`
    /// + feather:          https://github.com/feathericons/feather         | `airplay`, `align-center`
    /// + material-design:  https://github.com/google/material-design-icons | `action/all_inbox`, `social/cake`
    pub icon : String,
    /// #### rewrite icon names for frontend
    /// 
    /// **Example `material-design`:** `action/all_inbox` to `all-inbox`
    /// 
    /// **Example Javascript:**
    /// ```html
    /// <COMPONENT_NAME name="all-inbox"></COMPONENT_NAME>
    /// ```
    pub name : String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ConfigComponent {
    /// #### name of component
    /// 
    /// **Javascript Rules** name needs to include `-` for example `my-icons`
    /// 
    /// **Example Javascript:**
    /// ```html
    /// <my-icons name="all-inbox"></my-icons>
    /// ```
    pub name : String,
    #[serde(rename = "uiName")]
    /// #### default icon name for `HTMLElement`
    pub ui_name : String,
    #[serde(rename = "uiSize")]
    /// #### default icon size for `HTMLElement`
    pub ui_size : String,
    #[serde(rename = "uiColor")]
    /// #### default icon color for `HTMLElement`
    /// 
    /// **it's not the color. just sets the `currentColor`**
    /// 
    /// chose between: `fill` | `stroke`
    pub ui_color : String,
}