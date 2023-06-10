# rust-cli-k8s-manifest-generator

This is a _VERY_ basic implementation that is meant to be extensible

## Getting started

install rust with rustup

## To Use

`cargo update && cargo build --release`
`./target/release/rust_k8s_manifest_generator --name <name> --image <image> --port <port number>`

This command will generate k8s manifests in a directory with the name of the app+ '-manifests'
