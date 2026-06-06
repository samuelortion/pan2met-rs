//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

#[test]
fn functional() -> pan2met-rs::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("pan2met-rs")?;
    cmd.args(["-i", "ACTGactg"]);

    let assert = cmd.assert();

    assert
        .success()
        .stderr(b"" as &[u8])
        .stdout(b"cagtCAGT\n" as &[u8]);

    Ok(())
}
