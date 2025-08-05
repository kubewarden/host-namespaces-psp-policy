#!/usr/bin/env bats

@test "Test deployment validation" {
	run kwctl run  --request-path test_data/deployment_request.json --settings-path test_data/settings.json annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
	[ $(expr "$output" : '.*"allowed":false.*') -ne 0 ]
 }
 
@test "Test pod validation" {
	run kwctl run  --request-path test_data/pod_request.json --settings-path test_data/settings.json annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
	[ $(expr "$output" : '.*"allowed":false.*') -ne 0 ]
 }
