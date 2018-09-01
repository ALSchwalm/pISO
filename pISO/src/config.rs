#[derive(Clone, Debug, Deserialize)]
pub struct UiConfig {
    pub size_step: u32,
    pub default_size: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WifiApConfig {
    pub ssid: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WifiClientNetworkConfig {
    pub ssid: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WifiConfig {
    pub client: Option<Vec<WifiClientNetworkConfig>>,
    pub ap: WifiApConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub user: UserConfig,
    pub wifi: WifiConfig,
    pub ui: UiConfig,
}

#[cfg(test)]
mod tests {
    use toml;
    use super::*;
    #[test]
    fn load_test() {
        let toml_str = r#"
          [ui]
          size_step=5
          default_size=50

          [user]
          name="piso"
          password="password"

          [[wifi.client]]
          ssid="home-ap"
          password="faz"

          [[wifi.client]]
          ssid="test"
          password="foobar"

          [wifi.ap]
          ssid="piso"
          password="piso"
        "#;

        let _: Config = toml::from_str(toml_str).unwrap();
    }

    #[test]
    fn load_with_no_wifi_client() {
        let toml_str = r#"
          [ui]
          size_step=5
          default_size=50

          [user]
          name="piso"
          password="password"

          [wifi.ap]
          ssid="piso"
          password="piso"
        "#;

        let _: Config = toml::from_str(toml_str).unwrap();
    }
}
