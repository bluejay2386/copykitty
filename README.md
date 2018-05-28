# copykitty
Pretty portable MOTD
This is a project that I created to help learn Rust, so the code quality probably isn't great.

It's meant to be pretty portable. It relies on GNU df, uname, and hostname all existing on the target computer. It also relies on the file /etc/os-release existing and containing at least PRETTY_NAME="distro name"
I do plan to add some extra functionality as I get better at Rust.
