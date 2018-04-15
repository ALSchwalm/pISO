use toml;

#[derive(Debug, Deserialize)]
struct WifiApConfig {
    ssid: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct WifiClientNetworkConfig {
    ssid: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct WifiConfig {
    client: Vec<WifiClientNetworkConfig>,
    ap: WifiApConfig,
}

#[derive(Debug, Deserialize)]
struct Config {
    wifi: WifiConfig,
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
