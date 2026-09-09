//! The checked-in contract is verified by the dedicated codegen command.

use std::process::Command;

#[test]
fn codegen_check_is_a_read_only_tool_command() {
    let status = Command::new(env!("CARGO"))
        .args(["run", "-p", "xtask", "--", "check-schema"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("start the codegen check command");

    assert!(status.success(), "the codegen check command must succeed");
}
