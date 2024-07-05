extern crate semver;

use std::ffi::OsStr;
use color_eyre::{eyre, Result};
use semver::Version;

use std::process::Command;
use std::str;
use indexmap::IndexMap;
use subprocess::{Exec, Redirection};

use prettytable::{Table, row};

/// Compares two semantic versions and returns their order.
///
/// This function takes two version strings, parses them into `semver::Version` objects, and compares them.
/// It returns an `Ordering` indicating whether the current version is less than, equal to, or
/// greater than the target version.
///
/// # Arguments
///
/// * `current` - A string slice representing the current version.
/// * `target` - A string slice representing the target version to compare against.
///
/// # Returns
///
/// * `Result<std::cmp::Ordering>` - The comparison result.
pub fn compare_semver(current: &str, target: &str) -> Result<std::cmp::Ordering> {
    let current = Version::parse(current)?;
    let target = Version::parse(target)?;

    Ok(current.cmp(&target))
}

/// Retrieves the installed Nix version as a string.
///
/// This function executes the `nix --version` command, parses the output to extract the version string,
/// and returns it. If the version string cannot be found or parsed, it returns an error.
///
/// # Returns
///
/// * `Result<String>` - The Nix version string or an error if the version cannot be retrieved.
pub fn get_nix_version() -> Result<String> {
    let output = Command::new("nix").arg("--version").output()?;

    let output_str = str::from_utf8(&output.stdout)?;
    let version_str = output_str
        .lines()
        .next()
        .ok_or_else(|| eyre::eyre!("No version string found"))?;

    // Extract the version substring using a regular expression
    let re = regex::Regex::new(r"\d+\.\d+\.\d+")?;
    if let Some(captures) = re.captures(version_str) {
        let version = captures
            .get(0)
            .ok_or_else(|| eyre::eyre!("No version match found"))?
            .as_str();
        return Ok(version.to_string());
    }

    Err(eyre::eyre!("Failed to extract version"))
}

pub fn print_header(title: &str, map: IndexMap<&str, String>) {
    let mut header = Exec::cmd("figlet")
        .args(&["-w", "220", "-f", "Banner3-D", &format!(" {title} ")])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()
        .expect("figlet output")
        .stdout_str();

    let mut table = Table::new();
    table.set_titles(row!["Option", "Value"]);
    for (k, v) in map {
        table.add_row(row![k, v]);
    }
    let mut table = table.to_string();

    let header_line_length = first_line_length(&header);
    let table_line_length = first_line_length(&table);
    let section_length_diff: isize = header_line_length as isize - table_line_length as isize;
    if section_length_diff > 0 {
        table = pad_string_lines(&table, table_line_length + (section_length_diff / 2) as usize)
    } else {
        header = pad_string_lines(&header, header_line_length + (-section_length_diff / 2) as usize)
    }

    lolcatify("echo", &[&format!("\n{header}\n{table}\n")]);
}

fn first_line_length(str: &str) -> usize {
    return if let Some(pos) = str.find('\n') {
        str[..pos].chars().count()
    } else {
        str.chars().count()
    };
}

fn pad_string_lines(str: &str, padding: usize) -> String {
    let help = str
        .lines()
        .map(|line| format!("{:>width$}", line, width = padding))
        .collect::<Vec<String>>()
        .join("\n");

    return help;
}

fn lolcatify(cmd: &str, args: &[impl AsRef<OsStr>]) {
    _ = {
        Exec::cmd(cmd)
            .args(args)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            | Exec::cmd("lolcat")
    }
        .join();
}
