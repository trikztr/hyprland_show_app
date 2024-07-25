# hyprland_show_app
Rust program to focus a specific application in hyprland based on its name (initialClass).

## Installation
```bash
git clone https://github.com/trikztr/hyprland_show_app.git
cd hyprland_show_app
cargo install --path .
```

## Usage
### CLI
```bash
hyprland_show_app Spotify
```

### Waybar
```json
{
    "module": {
	    "on-click": "hyprland_show_app Application"
    }
}
```
