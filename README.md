# ZenithIDE

A lightweight, high-performance Integrated Development Environment built with Rust (Tauri v2) and SvelteKit. It combines the familiar user experience of modern editors with the power of native performance, aiming to provide a seamless coding environment for C, C++, Rust, and Web development without the bloat of traditional IDEs.

## Table of Contents

- [Features](#features)
- [System Requirements](#system-requirements)
- [Usage Guide](#usage-guide)
- [Comparison](#comparison)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Features

- **Native Performance:** Powered by a Rust backend, ensuring a low memory footprint and high responsiveness. The application runs natively using the OS's webview rather than bundling a full browser engine.
- **Dynamic Compiler Management:** Users can download and install C/C++ (MinGW) and Rust (rustup) compilers directly within the UI with a single click. No manual environment configuration is required.
- **Integrated AI Assistant:** A built-in panel allows developers to connect to any OpenAI-compatible API (or local AI models) for code generation, debugging, and explanations without leaving the editor.
- **Monaco Editor:** Utilizes the same editor engine that powers Visual Studio Code, ensuring familiar syntax highlighting, code completion, and text manipulation behaviors.
- **Integrated Terminal:** A full-featured pseudo-terminal (PTY) supporting interactive shell commands, build outputs, and standard input/output streams.
- **Multilingual User Interface:** Supports English and Vietnamese out of the box, with the ability to expand to more languages.
- **Cross-Platform:** Builds available for Windows (.exe), Linux (.deb, .AppImage), and macOS (.dmg).

## System Requirements

To run ZenithIDE efficiently, your system must meet the following minimum requirements:

- **Operating System:**
  - Windows 10 or later (64-bit).
  - Linux: Ubuntu 22.04 or equivalent (requires `libwebkit2gtk-4.1` installed).
  - macOS: 11.0 Big Sur or later.
- **RAM:** 2GB minimum (4GB recommended for compiling large Rust or C++ projects).
- **Storage:** 50MB for the IDE installation. An additional 1GB is required if you choose to download C/C++ and Rust compilers via the in-app downloader.
- **Network:** An active internet connection is required for downloading compilers and using the AI Assistant features.

## Usage Guide

### 1. Workspace Initialization
Upon launching, ZenithIDE automatically creates a `zenith_workspace` directory in your system's local AppData folder. This directory is pre-populated with sample `Hello World` files for C, C++, Rust, and HTML.

### 2. Running Code
- Open a file (`.c`, `.cpp`, `.rs`, or `.html`) from the Explorer sidebar.
- Click the "Run" button located in the top-right corner of the editor toolbar.
- If the required compiler is not found, the integrated terminal will prompt you to install it. Click the respective "Setup" button (e.g., Setup C/C++) in the toolbar. The IDE will download, extract, and configure the compiler automatically.
- HTML files will automatically open in your system's default web browser.

### 3. Using the AI Assistant
- Click the "AI" tab located next to the Terminal tab at the bottom of the screen.
- Enter your API Key (e.g., OpenAI API key) in the password-protected input field.
- Type your query or request in the text area and click "Ask AI". The response will be displayed in the panel above.

## Comparison

| Feature | ZenithIDE | Visual Studio Code | Dev-C++ | Zed |
| :--- | :--- | :--- | :--- | :--- |
| **Core Language** | Rust / Svelte | TypeScript / Electron | C++ | Rust |
| **Memory Footprint** | Very Low (~30-50MB) | High (~200-300MB) | Low | Very Low |
| **Built-in Compiler** | Yes (On-demand download) | No (Manual setup required) | Yes (Bundled) | No |
| **Integrated AI** | Yes (OpenAI-compatible) | Via Extensions | No | Yes |
| **Cross-Platform** | Yes | Yes | No (Windows only) | Yes |
| **Target Audience** | C/C++/Rust/Web Beginners & Pros | General Purpose | C/C++ Beginners | Web Developers |

*ZenithIDE bridges the gap between lightweight editors and heavy IDEs. Unlike VS Code, it does not require manual environment variable configuration for C/C++ compilers. Unlike Dev-C++, it provides a modern, cross-platform experience with AI integration.*

## License

This project is licensed under the Mozilla Public License Version 2.0 (MPL 2.0). 

You are free to use, modify, and distribute this software. Any modifications to the source code files must be made available under the same MPL 2.0 license. For the full license text, please refer to the `LICENSE` file included in the repository.

## Acknowledgements

The entire codebase, architecture, and CI/CD pipeline of ZenithIDE were entirely generated and engineered by **GLM 5.2** (Zhipu AI), demonstrating advanced capabilities in full-stack development combining Rust, Tauri, and SvelteKit.
