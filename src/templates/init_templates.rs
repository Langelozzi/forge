pub const FORGE_TOML: &str = r#"[project]
name = "{name}"
version = "0.1.0"

[build]
compiler = "{compiler}"
flags = []
ignore_files = []
"#;

pub const MAIN_C: &str = r#"#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("Hello World!\n");
    return 0;
}
"#;
