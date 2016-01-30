pub const ONE_TEXT_LINE_RS: &'static str = "//@ This is a demo without code.";
pub const ONE_TEXT_LINE_MD: &'static str = "This is a demo without code.";

pub const ONE_RUST_LINE_RS: &'static str = r#"fn main() { println!("one rust line"); }"#;

pub const ONE_RUST_LINE_MD: &'static str = r#"```rust
fn main() { println!("one rust line"); }
```
"#;

pub const HELLO_RS: &'static str = r#"//@ # Hello World
//@ This is a Hello World demo.

// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
//@ And then the text resumes here.
"#;

pub const HELLO_MD: &'static str = r#"# Hello World
This is a Hello World demo.

```rust
// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
```
And then the text resumes here.
"#;

pub const HELLO2_RS: &'static str = r#"//@ # Hello World
//@ This is a second Hello World demo.

// Code started here (at this normal comment)
fn main() { println!("Hello World"); }

//@ And then the text resumes here, after a line break.
"#;

pub const HELLO2_MD: &'static str = r#"# Hello World
This is a second Hello World demo.

```rust
// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
```

And then the text resumes here, after a line break.
"#;

pub const HELLO3_RS: &'static str = r#"

// Code started here (at this normal comment)
fn main() { hello() }

//@ Here is some expository text in the middle
//@ It spans ...
//@ ... multiple lines

// Here is yet more code!
// (and we end with code, not doc)
fn hello() { println!("Hello World"); }
"#;

pub const HELLO3_MD: &'static str = r#"

```rust
// Code started here (at this normal comment)
fn main() { hello() }
```

Here is some expository text in the middle
It spans ...
... multiple lines

```rust
// Here is yet more code!
// (and we end with code, not doc)
fn hello() { println!("Hello World"); }
```
"#;

pub const HELLO4_MD: &'static str = r#"# Hello World
Here is some expository text, but this one ...

... has a gap between its lines.
"#;

pub const HELLO4_RS: &'static str = r#"//@ # Hello World
//@ Here is some expository text, but this one ...
//@
//@ ... has a gap between its lines.
"#;

pub const PRODIGAL5_MD: &'static str = r#"# Hello World
```rust
let code_fragment;
```
	
This looks like it has a nice para break before its starts,
but note the tab
"#;

pub const HARVEST5_RS: &'static str = r#"//@ # Hello World
let code_fragment;
//@ 	
//@ This looks like it has a nice para break before its starts,
//@ but note the tab
"#;

pub const RETURN5_MD: &'static str = r#"# Hello World
```rust
let code_fragment;
```

This looks like it has a nice para break before its starts,
but note the tab
"#;

pub const HELLO6_METADATA_MD: &'static str = r#"# Hello World

```rust { .css_class_metadata }
// The question is, can we preserve the .css_class_metdata
```
"#;

pub const HELLO6_METADATA_RS: &'static str = r#"//@ # Hello World

//@@ { .css_class_metadata }
// The question is, can we preserve the .css_class_metdata
"#;

use super::{core_test_md2rs, core_test_rs2md};

#[test]
fn test_onetext_md2rs() {
    core_test_md2rs(ONE_TEXT_LINE_MD,
                    ONE_TEXT_LINE_RS);
}

#[test]
fn test_onetext_rs2md() {
    core_test_rs2md(ONE_TEXT_LINE_RS,
                    ONE_TEXT_LINE_MD);
}

#[test]
fn test_onerust_md2rs() {
    core_test_md2rs(ONE_RUST_LINE_MD,
                    ONE_RUST_LINE_RS);
}

#[test]
fn test_onerust_rs2md() {
    core_test_rs2md(ONE_RUST_LINE_RS,
                    ONE_RUST_LINE_MD);
}

#[test]
fn test_hello_md2rs() {
    core_test_md2rs(HELLO_MD, HELLO_RS);
}

#[test]
fn test_hello_rs2md() {
    core_test_rs2md(HELLO_RS, HELLO_MD);
}

#[test]
fn test_hello2_md2rs() {
    core_test_md2rs(HELLO2_MD, HELLO2_RS);
}

#[test]
fn test_hello2_rs2md() {
    core_test_rs2md(HELLO2_RS, HELLO2_MD);
}

#[test]
fn test_hello3_md2rs() {
    core_test_md2rs(HELLO3_MD, HELLO3_RS);
}

#[test]
fn test_hello3_rs2md() {
    core_test_rs2md(HELLO3_RS, HELLO3_MD);
}

#[test]
fn test_hello4_md2rs() {
    core_test_md2rs(HELLO4_MD, HELLO4_RS);
}

#[test]
fn test_hello4_rs2md() {
    core_test_rs2md(HELLO4_RS, HELLO4_MD);
}

#[test]
fn test_prodigal5_md2rs() {
   core_test_md2rs(PRODIGAL5_MD, HARVEST5_RS);
}

#[test]
fn test_prodigal5return_md2rs() {
   core_test_rs2md(HARVEST5_RS, RETURN5_MD);
}

#[test]
fn test_hello6_metadata_md2rs() {
    core_test_md2rs(HELLO6_METADATA_MD,
                    HELLO6_METADATA_RS);
}

#[test]
fn test_hello6_metadata_rs2md() {
    core_test_rs2md(HELLO6_METADATA_RS,
                    HELLO6_METADATA_MD);
}
