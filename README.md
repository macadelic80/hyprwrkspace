# HyprWorkspace

**HyprWorkspace** is a Rust-based tool for managing windows and workspaces in Hyprland. It allows users to automate the management of applications, assign customizable keyboard shortcuts, and handle multiple instances of the same application. All configurations are managed through a TOML file, which is dynamically read and updated.

## Features

- **Application Shortcuts**: Assign custom keyboard shortcuts to launch or focus specific applications.
- **Multiple Instances**: Open multiple instances of the same application, each associated with a different shortcut.
- **Dynamic Configuration**: Configuration is managed via a TOML file, which can be updated in real-time.
- **Automatic Workspace Management**: Automatically assign special workspaces to applications and manage window focus based on user preferences.

## Installation

### Prerequisites

- Rust installed on your machine.
- Hyprland window manager.

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/HyprWorkspace.git
   cd HyprWorkspace
