# copykitty
Pretty portable MOTD

![screenshot of output](https://i.redditmedia.com/OAyJJ1VyD981CWU6n7evGaq-nN-MKHtAZBk6V5jUeuY.png?s=9443169114d152e2acd860b2fbf70341)

This is a project that I created to help learn Rust, so the code quality probably isn't great.

It's meant to be pretty portable. It relies on GNU df, uname, and hostname all existing on the target computer. It also relies on the file /etc/os-release existing and containing at least PRETTY_NAME="distro name"
I do plan to add some extra functionality as I get better at Rust.

**Installing**
 + get the source code
 + build the binary with cargo run --release
 + this will be target/release/copykitty
 + copy the binary to the directory /etc/update-motd.d/
 + test the program with run-parts /etc/update-motd.d/ --test and run-parts /etc/update-motd.d/
 + This program depends on GNU df, hostname, uname, and Rust. 
