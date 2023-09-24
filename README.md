# 🖤 room

A [Zellij](https://zellij.dev) plugin for quickly searching
and switching between tabs.

![usage](https://github.com/rvcas/room/raw/main/img/usage.gif)

## Usage

- `Tab` to cycle through tab list
- `Up` and `Down` to cycle through tab list
- `Enter` to switch to the selected tab
- Start typing to filter the tab list
- `Esc` or `Ctrl + c` to exit

## Why?

I rename my tabs so once I have a lot of them I have to start
counting and then press `Ctrl + t` then `<tab num>`. So I wanted something
that let’s me type to filter the tab list and then press enter to jump to the selected tab.

## Installation

You'll need [rust](https://rustup.rs/) installed.

- `git clone git@github.com:rvcas/room.git`
- `cd room`
- `rustup target add wasm32-wasi && cargo build --release`
- `mkdir -p ~/.config/zellij/plugins/`
- `mv target/wasm32-wasi/release/room.wasm ~/.config/zellij/plugins/`

## Keybinding

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html)
somewhere inside the [keybinds](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl y" {
        LaunchOrFocusPlugin "file:~/.config/zellij/plugins/room.wasm" {
            floating true
            ignore_case true
        }
    }
}
```

> You likely already have a `shared_except "locked"` section in your configs. Feel free to add `bind` there.


The `ignore_case` defaults to `false` if absent. If set to `true`, filtering the tab names ignores
the case of the filter string and the tab name.

## Contributing

If you find any issues or want to suggest ideas please [open an issue](https://github.com/rvcas/room/issues/new).

### Development

Make sure you have [rust](https://rustup.rs/) installed then run:

```sh
zellij action new-tab --layout ./dev.kdl
```
