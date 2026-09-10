# BeZieR

An interactive terminal-based Bezier curve visualization and editing tool built with Rust. BeZieR allows you to create, manipulate, and visualize various spline types including points, lines, quadratic Bezier curves, and cubic Bezier curves directly in your terminal.

## Features

- **Interactive Canvas**: Click to add points and visualize splines in real-time
- **Multiple Spline Types**: Support for points, lines, quadratic Bezier curves, and cubic Bezier curves
- **Point Grouping**: Group points together to create different spline types
- **Mouse Support**: Full mouse interaction for adding, selecting, and deleting points
- **Keyboard Controls**: Intuitive keyboard shortcuts for all operations
- **Terminal UI**: Built with ratatui for a clean, responsive interface
- **Color Rendering**: RGB color support for points and lines

## Dependencies

- **Rust**: Edition 2024
- **ratatui**: 0.29 - Terminal UI framework
- **crossterm**: 0.29 - Cross-platform terminal manipulation

## Installation

### Prerequisites

Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd BeZieR

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Development Build

```bash
cargo build
cargo run
```

## Usage

### Starting the Application

```bash
cargo run
```

Or for the optimized release version:

```bash
cargo run --release
```

### Command Line Arguments

- `-d, --debug, -D, --DEBUG`: Enable debug mode (prints CLI arguments)
- `-h, --help, -H, --HELP`: Display help information

### Controls

#### Tools (Keyboard Shortcuts)

- **A** - Add Point tool (default)
- **S** - Select Point tool
- **D** - Delete Point tool
- **1** - Group 1 tool (creates Point groups)
- **2** - Group 2 tool (creates Line groups)
- **3** - Group 3 tool (creates Quadratic Bezier groups)
- **4** - Group 4 tool (creates Cubic Bezier groups)

#### Actions

- **G** - Create Group (creates a group based on current tool)
- **R** - Remove Group (removes selected group)
- **X** - Delete Selected Point
- **C** - Clear All (resets canvas)
- **Q** - Quit application

#### Mouse Controls

- **Left Click** (on canvas): Add point (when Add Point tool is selected)
- **Left Click** (on canvas): Select point (when Select tool is selected)
- **Left Click** (on canvas): Delete point (when Delete tool is selected)

## Examples

### Creating a Line

1. Press **A** to select Add Point tool
2. Click twice on the canvas to add two points
3. Press **2** to select Line group tool
4. Press **G** to create a line from the last two points

### Creating a Quadratic Bezier Curve

1. Press **A** to select Add Point tool
2. Click three times on the canvas to add three points
3. Press **3** to select Quadratic Bezier group tool
4. Press **G** to create a quadratic Bezier curve from the last three points

### Creating a Cubic Bezier Curve

1. Press **A** to select Add Point tool
2. Click four times on the canvas to add four points
3. Press **4** to select Cubic Bezier group tool
4. Press **G** to create a cubic Bezier curve from the last four points

### Deleting Points

1. Press **S** to select Select tool
2. Click on a point to select it
3. Press **X** to delete the selected point

Or:

1. Press **D** to select Delete tool
2. Click directly on a point to delete it

### Clearing the Canvas

Press **C** to clear all points and groups from the canvas.

## Architecture

### Modules

- **main.rs**: Entry point and CLI argument parsing
- **model.rs**: Core data structures and mathematical operations
  - `Point`: 2D point with linear interpolation
  - `QuadraticBezier`: Quadratic Bezier curve implementation
  - `CubicBezier`: Cubic Bezier curve implementation
  - `RGB`: Color representation
  - `Rasterizer`: Canvas rendering engine
  - `SplineType`: Enum for different spline types
- **ui.rs**: Terminal user interface and event handling
  - `AppState`: Application state management
  - `Tool`: Tool selection enum
  - Interactive canvas rendering
  - Mouse and keyboard event handling
- **helper.rs**: CLI argument parsing and helper functions

### Bezier Curve Mathematics

The application uses De Casteljau's algorithm for Bezier curve evaluation:

- **Linear Interpolation**: `P(t) = (1-t)P₀ + tP₁`
- **Quadratic Bezier**: Evaluated by recursively interpolating between three control points
- **Cubic Bezier**: Evaluated by recursively interpolating between four control points

## License

This project is licensed under the GPL-3.0 license. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Troubleshooting

### Terminal Size Issues

If the canvas appears cut off, ensure your terminal is at least 80x40 characters. Resize your terminal to accommodate the UI.

### Mouse Capture Issues

If mouse interactions don't work, ensure your terminal supports mouse events. Most modern terminals (iTerm2, GNOME Terminal, Windows Terminal, etc.) support this feature.

### Build Errors

If you encounter build errors, ensure you have the latest version of Rust:

```bash
rustup update
```
