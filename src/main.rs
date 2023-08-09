use std::{
    fs::{self, File},
    io::Read,
};

use anyhow::{bail, Context};
use classfile_parser::class_parser;
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    for filepath in std::env::args().skip(1) {
        let filepath = &filepath;

        if filepath.ends_with(".class") {
            let classfile_bytes = fs::read(filepath)
                .with_context(|| format!("Failed to read java class from {}", filepath))?;

            let (major_version, minor_version) = parse_class_version(&classfile_bytes)?;

            println!(
                "{}: compiled Java class data, version {}.{}",
                filepath, major_version, minor_version
            );
        } else if filepath.ends_with(".jar") {
            let err_f = || format!("Failed to read java class from {}", filepath);

            let archive = File::open(filepath).with_context(err_f)?;
            let mut zip = ZipArchive::new(archive).with_context(err_f)?;

            let mut major_version = 0;
            let mut minor_version = 0;

            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;

                // Package name cannot contain `-` https://docs.oracle.com/javase/specs/jls/se17/html/jls-6.html#d5e8745
                // We should filter entry name to deal with the edge case
                if file.name().ends_with(".class") {
                    let classfile_bytes = {
                        let mut classfile_bytes: Vec<u8> = Vec::new();
                        file.read_to_end(&mut classfile_bytes).with_context(|| "")?;
                        classfile_bytes
                    };
                    let (major_version_c, minor_version_c) = parse_class_version(&classfile_bytes)?;
                    major_version = std::cmp::max(major_version, major_version_c);
                    if major_version == major_version_c && minor_version_c > minor_version {
                        minor_version = minor_version_c;
                    }
                }

                // TODO: respect multi-release Jar file (pick the one with lowest major_version and minor_version)
            }

            println!(
                "{}: Java archive data (JAR), max class version {}.{}",
                filepath, major_version, minor_version
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

fn parse_class_version(classfile_bytes: &[u8]) -> anyhow::Result<(u16, u16)> {
    let (major_version, minor_version) = class_parser(classfile_bytes)
        .map(|(_, class)| (class.major_version, class.minor_version))
        .map_err(|err| err.to_owned())?;
    Ok((major_version, minor_version))
}
