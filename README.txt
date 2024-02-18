Don't expect this to work outside of x64 Linux.

Usage: `bfjit-rs [memory size in bytes] [code]`

Examples:
Compiling: `cargo build --release`
Running in debug mode: `cargo run -- 10 "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."`
Running after build: `target/release/bfjit-rs 10 "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."`
