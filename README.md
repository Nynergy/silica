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
give it some more parameters to customize its look/behavior. Here are all the
optional parameters, which can also be seen by using the `-h` or `--help`
options:

| Parameter      | Value(s)                    | Defaults | Description |
|----------------|-----------------------------|----------|-------------|
| --text, -t     | String                      | None     | Text that will appear above the timer |
| --post-text    | String                      | None     | Text that will appear above the timer after it has elapsed |
| --digit\_size  | Enum (small, medium, large) | medium   | Size of the digits of the timer |
| --digit\_color | Int (0-8)                   | 2        | Color of the digits of the timer |
| --text\_color  | Int (0-8)                   | 7        | Color of the text above the timer |
| --ascii\_color | Int (0-8)                   | 7        | Color of the ascii art beside the timer |
| --blink\_color | Int (0-8)                   | 1        | Color of the blinking digits of the timer |
| --noblink      | bool                        | false    | Prevent the timer from blinking after it has elapsed |
| --noascii      | bool                        | false    | Prevent the ascii art from being rendered |

The numbers specified for the various colors are taken as indexes into the
colors of the terminal, sometimes set using something like `.Xresources`. An
example that uses every single argument is as follows:

```bash
$ silica 500 -t "This text appears first!" --post-text "This text appears after the time is up!" --digit-size large --digit-color 4 --text-color 5 --ascii-color 3 --blink-color 2 --noblink --noascii
```

In the above example, we set a timer for 500 seconds, give it text to show
before and after it has elapsed, tell it to render the timer digits in the large
size, make the digits use color 4 (typically blue), make the text use color 5
(typically magenta), make the ascii use color 3 (typically yellow), make the
blinking digits use color 2 (typically green), prevent the digits from blinking,
and prevent the ascii art from rendering. In this case, some of our color
arguments are unnecessary given that they will never be rendered, but this is
just a contrived example.

## Now what?

Use it, and know how much time you have left :)
