#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/guide/index.md"))]
#![doc(alias = "user guide")]

#[doc(alias = "getting started")]
pub mod getting_started {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/getting_started.md"
    ))]
}

pub mod configuration {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/configuration.md"
    ))]
}

pub mod connector {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/connector.md"
    ))]
}

pub mod input {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/input.md"
    ))]
}

pub mod output {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/output.md"
    ))]
}

pub mod data {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/data.md"
    ))]
}

pub mod errors {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/errors.md"
    ))]
}

pub mod threading {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/threading.md"
    ))]
}

pub mod advanced {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/advanced.md"
    ))]
}
