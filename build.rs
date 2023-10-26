extern crate prost_build;

use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();

    prost_build.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    prost_build.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    prost_build.include_file("_includes.rs");

    prost_build.compile_protos(
        &[
            "k8s.io/api/core/v1/generated.proto",
            "k8s.io/api/admissionregistration/v1/generated.proto",
            "k8s.io/api/apps/v1/generated.proto",
            "route/v1/generated.proto",
        ],
        &["./src/protobuf"],
    )?;

    Ok(())
}