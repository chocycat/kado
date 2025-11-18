# kado

X11 hotspot triggers for executing commands when your cursor enters screen corners and/or edges

## Installation

```
git clone https://github.com/chocycat/kado.git
cd kado
cargo install --path .
```

## Configuration

Create `~/.config/kado.toml`:

```toml
refresh_rate = 120

# global hotspots, these happen on all monitors
[top]
on_enter = "notify-send 'top'"
size = 10

[bottom]
on_enter = "rofi -show drun"
delay = 100 # waits 100ms before triggering

# screen-specific hotspots
# see the output of xrandr for your display name
['DisplayPort-0'.top]
on_enter = "notify-send 'top on DisplayPort-0'"
```

#### Positions

Valid options are: `top`, `bottom`, `left`, `right`, `top_left`, `top_right`, `bottom_left`, `bottom_right`

#### Options

- `on_enter` - command to run when entering hotspot
- `on_leave` - command to run when leaving hotspot
- `delay` - milliseconds cursor must stay in position before triggering (default: `0`)
- `size` - hotspot size in pixels (default: `0`)
- `enabled` - enable/disable hotspot (default: `true`)

## Usage

```bash
kado                    # uses ~/.config/kado.toml
kado --config path.toml # uses custom config location
```
