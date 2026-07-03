use std::{env, path::PathBuf};

use schema_rust::build::{CargoSchemaMetadata, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        CargoSchemaMetadata::new("signal-cloud").emit_schema_directory(&self.crate_root);
        GenerationDriver::new(GenerationPlan::wire_contract(
            &self.crate_root,
            "signal-cloud",
            "0.1.0",
        ))
        .generate()
        .expect("generate signal-cloud schema artifacts")
        .write_or_check("SIGNAL_CLOUD_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in signal-cloud schema artifacts are fresh");
    }
}
