use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct PortRange {
    pub min: i32,
    pub max: i32,
}

impl kubewarden::settings::Validatable for PortRange {
    fn validate(&self) -> Result<(), String> {
        if self.min > self.max {
            return Err(format!(
                "port min {} cannot be greater than max {}",
                self.min, self.max
            ));
        }
        Ok(())
    }
}

impl PortRange {
    pub fn in_range(&self, port: i32) -> bool {
        port >= self.min && port <= self.max
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub allow_host_ipc: bool,
    pub allow_host_network: bool,
    pub allow_host_pid: bool,
    pub allow_host_ports: Vec<PortRange>,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        for port_range in self.allow_host_ports.iter() {
            port_range.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() {
        let invalid_settings = Settings {
            allow_host_ports: vec![PortRange { min: 443, max: 80 }],
            ..Default::default()
        };
        assert!(!invalid_settings.validate().is_ok());

        let valid_settings = Settings {
            allow_host_ports: vec![PortRange { min: 80, max: 443 }],
            ..Default::default()
        };
        assert!(valid_settings.validate().is_ok());

        let valid_settings = Settings {
            allow_host_ports: vec![
                PortRange { min: 80, max: 80 },
                PortRange { min: 443, max: 443 },
            ],
            ..Default::default()
        };
        assert!(valid_settings.validate().is_ok());
    }
}
