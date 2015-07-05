# NSA - sketchy words on the command line

This tool reads random lines from `~/.nsa` and prints them to STDOUT. It's
inspired by Emacs's `m-x spook` command and is intended to be used with a file
like [spook.lines] or your own list of sketchy words.

[spook.lines]: https://github.com/emacs-mirror/emacs/blob/7f01832e1360b5203695d48605a45228f1362b42/etc/spook.lines

If you're happy with those words, you can run the following to add them to your
`~/.nsa` file: `curl -fsSL http://pastebin.com/raw.php?i=MPJifFAk > ~/.nsa`

## Installing

Before you get started, make sure you have [Rust] and [Cargo] installed.

1. Clone this repo.
2. Run `cargo build --release`.
3. Move `target/release/nsa` to somewhere in your path. eg: `/usr/local/bin`.

[Rust]: http://www.rust-lang.org/
[Cargo]: http://crates.io/

## Usage

Here's some quick and easy ways to potentially get put on a watch list:

* Vim - `:read !nsa`
* OS X clipboard - `nsa | pbcopy`
* Linux clipboard - `nsa | xclip`
* Email - `nsa | mail -s subject blake@blakewilliams.me`
* Mutt - `nsa | -s subject blake@blakewilliams.me`
* Twitter - `nsa | t update`

## Contributing

Want to improve the `nsa`, or have another quick and easy way to get put on a
watch list?

1. Fork this repo.
2. Create a topic branch.
3. Commit your changes.
3. Submit a pull request.

## Why?

A friend showed me `m-x spook` and I thought it was great. That combined with a
desire to write something in Rust led me to write this.
