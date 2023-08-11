## javav-rs

Displays version information for `.class` files (major.minor) and `.jar` files (max class version inside jar). One possible use case is to determine the minimum JVM version required to run given `.class` or `.jar` files.

Support Windows, macOS, and Linux.


## Installation

You can download the pre-build artifacts on the [Release](https://github.com/yuxqiu/javav-rs/releases) page.


## Usage

```console
$ ./javav-rs
A tool to display version information for .class files and .jar files (max class version inside jar).

Usage: ./javav-rs [file ...]
```

```console
$ ./javav-rs Main.class Main.jar
Main.class: compiled Java class data, version 61.0
Main.jar: Java archive data (JAR), max class version 52.0
```


## Build

Clone the repository:

    git clone https://github.com/yuxqiu/javav-rs.git

Then, run:

    cargo build --release


## Feature/Plan

- [x] Display version information for `.class` file
- [x] Display max class version for `.jar` file
- [ ] Respect multi-release Jar file
- [ ] Use custom `.class` parser that supports latest `.class` file specification