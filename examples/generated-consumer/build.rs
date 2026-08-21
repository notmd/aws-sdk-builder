fn main() -> Result<(), Box<dyn std::error::Error>> {
    aws_sdk_build::configure()
        .model("model/service.json")
        .service("com.example#Example")
        .operations(["GetThing"])
        .out_dir(std::env::var_os("OUT_DIR").unwrap())
        .compile()?;
    Ok(())
}
