use std::{
    fs::{self, File},
    io::Read,
};

use anyhow::{bail, Context};
use cafebabe::{parse_class_with_options, ParseOptions};
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    for filepath in std::env::args().skip(1) {
        let filepath = &filepath;
        let op = {
            let mut op = ParseOptions::default();
            op.parse_bytecode(false);
            op
        };
        let f_fail_to_read = || format!("Failed to read java class from {}", filepath);

        if filepath.ends_with(".class") {
            let classfile_bytes = fs::read(filepath).with_context(f_fail_to_read)?;

            let (major_version, minor_version) = parse_class_with_options(&classfile_bytes, &op)
                .map(|class| (class.major_version, class.minor_version))
                .with_context(|| format!("Failed to parse java class from {}", filepath))?;

            println!(
                "{}: compiled Java class data, version {}.{}",
                filepath, major_version, minor_version
            );
        } else if filepath.ends_with(".jar") {
            let archive = File::open(filepath).with_context(f_fail_to_read)?;
            let mut zip = ZipArchive::new(archive).with_context(f_fail_to_read)?;

            let mut major_version = 0;
            let mut minor_version = 0;

            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;

                if file.name().ends_with(".class") {
                    let classfile_bytes = {
                        let mut classfile_bytes: Vec<u8> = Vec::new();
                        file.read_to_end(&mut classfile_bytes).with_context(|| "")?;
                        classfile_bytes
                    };

                    let (major_version_c, minor_version_c) =
                        parse_class_with_options(&classfile_bytes, &op)
                            .map(|class| (class.major_version, class.minor_version))
                            .with_context(|| {
                                format!(
                                    "Failed to parse java class from entry {} of {}",
                                    file.name(),
                                    filepath,
                                )
                            })?;

                    major_version = std::cmp::max(major_version, major_version_c);
                    if major_version == major_version_c && minor_version_c > minor_version {
                        minor_version = minor_version_c;
                    }
                }

                // Package name cannot contain `-` and other forbidden characters
                // https://docs.oracle.com/javase/specs/jls/se17/html/jls-6.html#d5e8745
                // https://docs.oracle.com/javase/specs/jls/se17/html/jls-3.html#jls-3.8
                //
                // The packaged jar file may contain some `.class` files that are not supposed to
                // be used and are placed in some strange folder. But Jar files packaged by
                // commonly-used build tools are unlikely to contain these files.
                // If this happens fairly often, we will implement a fix to deal with this situation.

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
