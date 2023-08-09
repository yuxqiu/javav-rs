## javav-rs

Displays version information for `.class` files (major.minor) and `.jar` files (max class version inside this jar). One possible use case is to determine the minimum JVM version required to run given `.class` or `.jar` files.

Support Windows, macOS, and Linux.


## Build

Clone the repository:

```sh
git clone https://github.com/yuxqiu/javav-rs.git
```

Then, run:

```
cargo build --release
```


## Feature/Plan

- [x] Display version information for `.class` file
- [x] Display max class version for `.jar` file
- [ ] Respect multi-release Jar file
- [ ] Use custom `.class` parser that supports latest `.class` file specification