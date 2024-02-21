Don't expect this to work outside of x64 Linux.

Usage:
$ bfjit-rs [memory size in bytes] [code]


Examples:

Running in debug mode:
$ cargo run -- 10 "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."`

Compiling release build:
$ cargo build --release

Running release build:
$ target/release/bfjit-rs 10 "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
