extern crate kubewarden_policy_sdk as kubewarden;
use kubewarden::{protocol_version_guest, request::ValidationRequest, validate_settings};

use guest::prelude::*;
use kubewarden::wapc_guest as guest;

use k8s_openapi::api::core::v1 as apicore;

mod settings;
use settings::Settings;

#[no_mangle]
pub extern "C" fn wapc_init() {
    register_function("validate", validate);
    register_function("validate_settings", validate_settings::<Settings>);
    register_function("protocol_version", protocol_version_guest);
}

fn validate(payload: &[u8]) -> CallResult {
    let validation_request: ValidationRequest<Settings> = ValidationRequest::new(payload)?;

    let pod = match serde_json::from_value::<apicore::Pod>(validation_request.request.object) {
        Ok(pod) => pod,
        Err(_) => return kubewarden::accept_request(),
    };

    let pod_spec = pod.spec.ok_or("invalid pod spec")?;
    let settings = validation_request.settings;

    if pod_spec.host_ipc.unwrap_or(false) && !settings.allow_host_ipc {
        return kubewarden::reject_request(
            Some("Pod has IPC enabled, but this is not allowed".to_string()),
            None,
            None,
            None,
        );
    }

    if pod_spec.host_network.unwrap_or(false) && !settings.allow_host_network {
        return kubewarden::reject_request(
            Some("Pod has host network enabled, but this is not allowed".to_string()),
            None,
            None,
            None,
        );
    }

    if pod_spec.host_pid.unwrap_or(false) && !settings.allow_host_pid {
        return kubewarden::reject_request(
            Some("Pod has host PID enabled, but this is not allowed".to_string()),
            None,
            None,
            None,
        );
    }

    if !all_containers_allowed(
        &pod_spec.init_containers.unwrap_or_default(),
        &settings.allow_host_ports,
    ) {
        return kubewarden::reject_request(
            Some("Pod is using unallowed host ports in init containers".to_string()),
            None,
            None,
            None,
        );
    }

    if !all_containers_allowed(&pod_spec.containers, &settings.allow_host_ports) {
        return kubewarden::reject_request(
            Some("Pod is using unallowed host ports in containers".to_string()),
            None,
            None,
            None,
        );
    }

    kubewarden::accept_request()
}

fn all_containers_allowed(
    containers: &[apicore::Container],
    allowed_host_ports: &[settings::PortRange],
) -> bool {
    containers.iter().all(|container| {
        container
            .ports
            .clone()
            .unwrap_or_default()
            .iter()
            .all(|port| match port.host_port {
                Some(host_port) => allowed_host_ports
                    .iter()
                    .any(|allowed_host_port| allowed_host_port.in_range(host_port)),
                None => true,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    use kubewarden_policy_sdk::test::Testcase;

    #[test]
    fn pod_host_ipc() -> Result<()> {
        let request_file = "test_data/pod_host_ipc_enabled.json";

        let tests = [
            Testcase {
                name: String::from("Host IPC namespace disallowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_ipc: false,
                    ..Default::default()
                },
                expected_validation_result: false,
            },
            Testcase {
                name: String::from("Host IPC namespace allowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_ipc: true,
                    ..Default::default()
                },
                expected_validation_result: true,
            },
        ];

        for tc in tests.iter() {
            tc.eval(validate)?;
        }

        Ok(())
    }

    #[test]
    fn pod_host_network() -> Result<()> {
        let request_file = "test_data/pod_host_network_enabled.json";

        let tests = [
            Testcase {
                name: String::from("Host network namespace disallowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_network: false,
                    ..Default::default()
                },
                expected_validation_result: false,
            },
            Testcase {
                name: String::from("Host network namespace allowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_network: true,
                    ..Default::default()
                },
                expected_validation_result: true,
            },
        ];

        for tc in tests.iter() {
            tc.eval(validate)?;
        }

        Ok(())
    }

    #[test]
    fn pod_host_pid() -> Result<()> {
        let request_file = "test_data/pod_host_pid_enabled.json";

        let tests = [
            Testcase {
                name: String::from("Host PID namespace disallowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_pid: false,
                    ..Default::default()
                },
                expected_validation_result: false,
            },
            Testcase {
                name: String::from("Host PID namespace allowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_pid: true,
                    ..Default::default()
                },
                expected_validation_result: true,
            },
        ];

        for tc in tests.iter() {
            tc.eval(validate)?;
        }

        Ok(())
    }

    #[test]
    fn pod_host_port() -> Result<()> {
        let request_file = "test_data/pod_host_ports_443.json";

        let tests = [
            Testcase {
                name: String::from("Host port 443 allowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_ports: vec![
                        settings::PortRange { min: 200, max: 300 },
                        settings::PortRange { min: 443, max: 443 },
                        settings::PortRange {
                            min: 8000,
                            max: 9000,
                        },
                    ],
                    ..Default::default()
                },
                expected_validation_result: true,
            },
            Testcase {
                name: String::from("Host port 443 allowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_ports: vec![settings::PortRange { min: 80, max: 443 }],
                    ..Default::default()
                },
                expected_validation_result: true,
            },
            Testcase {
                name: String::from("Host port 443 disallowed"),
                fixture_file: String::from(request_file),
                settings: Settings {
                    allow_host_ports: vec![settings::PortRange {
                        min: 8000,
                        max: 9000,
                    }],
                    ..Default::default()
                },
                expected_validation_result: false,
            },
        ];

        for tc in tests.iter() {
            tc.eval(validate)?;
        }

        Ok(())
    }
}
