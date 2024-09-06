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
- Quick jump to a tab by pressing it's displayed number

> To enable quick jumps, you need to set the config option for it to `quick_jump true`. The downside is that you won't be able to properly filter down tabs that have a number in their name.

## Why?

I rename my tabs so once I have a lot of them I have to start
counting and then press `Ctrl + t` then `<tab num>`. So I wanted something
that let’s me type to filter the tab list and then press enter to jump to the selected tab.

## Installation

Download `room.wasm` from the [latest release](https://github.com/rvcas/room/releases/latest)

- `mkdir -p ~/.config/zellij/plugins/`
- `mv room.wasm ~/.config/zellij/plugins/`

> You don't need to keep `room.wasm` at this specified location. It's just where I like to
> keep my zellij plugins.

### Quick Install

```
mkdir -p ~/.config/zellij/plugins && \
  curl -L "https://github.com/rvcas/room/releases/latest/download/room.wasm" -o ~/.config/zellij/plugins/room.wasm
```

## Keybinding

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html)
somewhere inside the [keybinds](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl y" {
        LaunchOrFocusPlugin "file:~/.config/zellij/plugins/room.wasm" {
            floating true
            ignore_case true
            quick_jump true
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
