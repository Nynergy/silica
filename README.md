# silica

A sand timer for the terminal

------------------------------------------------------------------------------

## What is silica and why does it exist?

_silica_ is a sand timer for the terminal. You give it a time in seconds to
count down from, and optionally some text to present alongside the time, and it
takes care of the rest. Once the specified time has expired, the timer will
blink on and off in order to be a bit more attention grabbing.

## How can I compile and run it?

First, you need to clone the repo:

```bash
$ git clone https://github.com/Nynergy/silica.git
$ cd silica
```

To build the app and mess around with it, run the following:

```bash
$ cargo build
$ cargo run
```

To install it into your Cargo installation's install root, do:

```bash
$ cargo install --path .
```

Just be sure that your Cargo install root is in your PATH environment variable,
otherwise your shell won't know where to find the installed binary.

To run it, you must provide it a time in seconds, and you may also optionally
give it some text to show alongside the time. For example, if you wanted to show
a 10 minute timer, you would invoke _silica_ like so:

```bash
$ silica 600
```

Let's say you wanted a timer for three minutes and twenty-seven seconds, and you
want it to say something fun alongside the time. Then you could do:

```bash
$ silica 207 --text "Ordering my monkeys to slam furiously on typewriters..."
```

You can also use `-t` in place of `--text` for the text argument.

## Now what?

Use it, and know how much time you have left :)
