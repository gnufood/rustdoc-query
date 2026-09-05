//! Repository maintenance commands.

#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::as_conversions
    )
)]
#![forbid(unsafe_code)]

mod schema_codegen;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let command = arguments.next().unwrap_or_default();
    let result = match command.as_str() {
        "generate-schema" => schema_codegen::run(schema_codegen::Mode::Generate),
        "check-schema" => schema_codegen::run(schema_codegen::Mode::Check),
        _ => Err("usage: cargo run -p xtask -- <generate-schema|check-schema>".to_owned()),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}
