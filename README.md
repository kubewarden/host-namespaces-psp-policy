# Kubewarden policy psp-host-namespaces

## Description

Replacement for the Kubernetes Pod Security Policy that controls the
usage of host namespaces

## Settings

This policy works by defining what host namespaces can be used by a Pod.

The following setting keys are accepted for this policy:

* `allow_host_ipc`: allows the pod to set `.spec.HostIPC` to true.
* `allow_host_network`: allows the pod to set `.spec.HostNetwork` to true.
* `allow_host_pid`: allows the pod to set `.spec.HostPID` to true.
* `allow_host_ports`: is a range of ports of the form:

  ```yaml
  allow_host_ports:
    - min: 80
      max: 80
    - min: 443
      max: 443
    - min: 8000
      max: 9000
  ```

  This example would allow host ports `80`, `443` and the range `8000-9000`.

`allow_host_ipc`, `allow_host_network` and `allow_host_pid` are
`false` by default. `allow_host_ports` is an empty list by
default. This means that by default host IPC, network, pid and all
host ports are disabled when this policy is loaded with no
configuration.

The policy validates Pods at creation time.

## License

```
Copyright (C) 2021 Rafael Fernández López <ereslibre@ereslibre.es>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
