/// Represents the library consumer's application information.
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub website: Option<String>,
}

impl ToString for AppInfo {
    fn to_string(&self) -> String {
        let mut app_info = format!("{} {}", self.name, self.version);

        if let Some(website) = &self.website {
            app_info.push_str(&format!(" ({website})"));
        }

        app_info
    }
}
