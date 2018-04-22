use toml;

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
    pub client: Vec<WifiClientNetworkConfig>,
    pub ap: WifiApConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub wifi: WifiConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_test() {
        let toml_str = r#"
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

        let decoded: Config = toml::from_str(toml_str).unwrap();
    }
}
