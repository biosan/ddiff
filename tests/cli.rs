use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::Builder;
use tempfile::TempDir;

#[test]
fn simple_dirs_comparsion() -> Result<(), Box<dyn std::error::Error>> {
    let dir_a = TempDir::new()?;
    let dir_b = TempDir::new()?;

    // TODO: Unfortunately tempfile does not allow to specify the exact file name
    //       (there is a random string in name)
    // let mut dir_a_equal_file = Builder::new().prefix("equalfile").tempfile_in(&dir_a)?;
    // let mut dir_b_equal_file = Builder::new().prefix("equalfile").tempfile_in(&dir_b)?;
    // writeln!(dir_a_equal_file, "equal")?;
    // writeln!(dir_b_equal_file, "equal")?;

    let mut dir_a_unique_file = Builder::new().prefix("uniqueA").tempfile_in(&dir_a)?;
    let mut dir_b_unique_file = Builder::new().prefix("uniqueB").tempfile_in(&dir_b)?;
    writeln!(dir_a_unique_file, "uniqueA")?;
    writeln!(dir_b_unique_file, "uniqueB")?;

    let mut dir_a_diff_file = Builder::new().prefix("diff").tempfile_in(&dir_a)?;
    let mut dir_b_diff_file = Builder::new().prefix("diff").tempfile_in(&dir_b)?;
    writeln!(dir_a_diff_file, "diffA")?;
    writeln!(dir_b_diff_file, "diffB")?;

    let mut cmd = Command::cargo_bin("ddiff")?;
    cmd.arg(&dir_a.path()).arg(&dir_b.path());

    cmd.assert().success().stdout(
        predicate::str::contains("diff")
            .and(predicate::str::contains("uniqueA"))
            .and(predicate::str::contains("uniqueB"))
            .and(predicate::str::contains("28 B")),
    );

    Ok(())
}
