use std::path::Path;

use aws_sdk_build::smithy::BuildConfig;

#[test]
fn smithy_config_contains_aws_rust_codegen_projection() {
    let config = BuildConfig::new(
        Path::new("model.json"),
        "example#WeatherService",
        Path::new("output"),
        "software.amazon.smithy.rust:codegen-aws-sdk:0.1.25",
    )
    .to_json();

    assert_eq!(config["version"], "1.0");
    assert_eq!(config["outputDirectory"], "output");
    assert_eq!(config["imports"][0], "model.json");
    assert_eq!(
        config["maven"]["dependencies"][0],
        "software.amazon.smithy.rust:codegen-aws-sdk:0.1.25"
    );

    let plugin = &config["projections"]["aws-sdk"]["plugins"]["rust-client-codegen"];
    assert_eq!(plugin["service"], "example#WeatherService");
    assert_eq!(plugin["module"], "weather_service_sdk");
    assert_eq!(plugin["codegen"]["includeFluentClient"], true);
    assert_eq!(
        plugin["customizationConfig"]["awsSdk"]["awsSdkBuild"],
        false
    );
    assert_eq!(
        plugin["customizationConfig"]["awsSdk"]["suppressReadme"],
        true
    );
}
