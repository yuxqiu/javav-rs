## javav-rs

A tool to find the minimum Java version required to run given `.class` and `.jar` files.

Support up to Java 17 on Windows, macOS, and Linux.


## Installation

You can download the pre-build artifacts on the [Release](https://github.com/yuxqiu/javav-rs/releases) page.


## Usage

```console
$ ./javav-rs
A tool to find the minimum Java version required to run given .class and .jar files.

Usage: ./javav-rs [file ...]
```

```console
$ ./javav-rs Main.class Main.jar
Main.class: compiled Java class data, require Java 17 or above
Main.jar: Java archive data (JAR), require Java 8 or above
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