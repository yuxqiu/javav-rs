use core::fmt;
use std::{
    fs::{self, File},
    io::Read,
};

use anyhow::{bail, Context, Ok};
use cafebabe::{parse_class_with_options, ParseOptions};
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 1 {
        println!(
            "\
        A tool to find the minimum Java version required to run given .class and .jar files.\n\n\
        Usage: {} [file ...]\
      ",
            &args[0]
        );
    }

    let op = {
        let mut op = ParseOptions::default();
        op.parse_bytecode(false);
        op
    };

    for filepath in &args[1..] {
        if filepath.ends_with(".class") {
            println!(
                "{}: compiled Java class data, require Java {} or above",
                filepath,
                get_java_version_from_classfile(filepath, &op)?
            );
        } else if filepath.ends_with(".jar") {
            println!(
                "{}: Java archive data (JAR), require Java {} or above",
                filepath,
                get_java_version_from_jarfile(filepath, &op)?
            );
        } else {
            bail!(
                "Got {}. Expect a file ends with '.class' or '.jar'",
                filepath
            );
        }
    }

    Ok(())
}

fn is_multi_release_jar(zip: &mut ZipArchive<File>) -> anyhow::Result<bool> {
    let mut manifest = zip
        .by_name("META-INF/MANIFEST.MF")
        .context("No manifest file is given")?;
    let content = {
        let mut content = String::new();
        manifest
            .read_to_string(&mut content)
            .context("Manifest file does not use utf-8 encoding")?;
        content
    };
    let mut lines = content.split('\n');

    Ok(lines.any(|line| line == "Multi-Release: true"))
}

#[repr(transparent)]
struct JavaVersion(u16);

impl fmt::Display for JavaVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn major_version_to_java_version(major_version: u16) -> Option<JavaVersion> {
    match major_version {
        45..=65 => Some(JavaVersion(major_version - 44)),
        _ => None,
    }
}

fn get_java_version_from_classfile(
    filepath: &str,
    op: &ParseOptions,
) -> anyhow::Result<JavaVersion> {
    let f_fail_to_read = || format!("Failed to read java class from {}", filepath);
    let classfile_bytes = fs::read(filepath).with_context(f_fail_to_read)?;

    let major_version = parse_class_with_options(&classfile_bytes, &op)
        .map(|class| class.major_version)
        .with_context(|| format!("Failed to parse java class from {}", filepath))?;

    Ok(
        major_version_to_java_version(major_version).with_context(|| {
            format!(
                "Unsupported major version {} from {}",
                major_version, filepath
            )
        })?,
    )
}

fn get_java_version_from_jarfile(filepath: &str, op: &ParseOptions) -> anyhow::Result<JavaVersion> {
    let f_fail_to_read = || format!("Failed to read java class from {}", filepath);
    let f_fail_to_get_version = || format!("Failed to get java version from {}", filepath);

    let archive = File::open(filepath).with_context(f_fail_to_read)?;
    let mut zip = ZipArchive::new(archive).with_context(f_fail_to_read)?;

    Ok(
        if is_multi_release_jar(&mut zip)
            .with_context(|| format!("Failed to parse manifest file from {}", filepath))?
        {
            get_java_version_from_simple_jar(&mut zip, &op).with_context(f_fail_to_get_version)?
        } else {
            get_java_version_from_simple_jar(&mut zip, &op).with_context(f_fail_to_get_version)?
        },
    )
}

fn get_java_version_from_simple_jar(
    zip: &mut ZipArchive<File>,
    op: &ParseOptions,
) -> anyhow::Result<JavaVersion> {
    let mut major_version = 0;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        if file.name().ends_with(".class") {
            let classfile_bytes = {
                let mut classfile_bytes: Vec<u8> = Vec::new();
                file.read_to_end(&mut classfile_bytes)?;
                classfile_bytes
            };

            let (major_version_c, _) = parse_class_with_options(&classfile_bytes, &op)
                .map(|class| (class.major_version, class.minor_version))
                .with_context(|| {
                    format!("Failed to parse java class from entry {}", file.name())
                })?;

            major_version = std::cmp::max(major_version, major_version_c);
        }

        // Package name cannot contain `-` and other forbidden characters
        // https://docs.oracle.com/javase/specs/jls/se17/html/jls-6.html#d5e8745
        // https://docs.oracle.com/javase/specs/jls/se17/html/jls-3.html#jls-3.8
        //
        // The packaged jar file may contain some `.class` files that are not supposed to
        // be used and are placed in some strange folder. But Jar files packaged by
        // commonly-used build tools are unlikely to contain these files.
        // If this happens fairly often, we will implement a fix to deal with this situation.
    }

    Ok(major_version_to_java_version(major_version)
        .with_context(|| format!("Unsupported major version {}", major_version))?)
}
