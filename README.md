# WebAssembly with Rust Example

## Introduction
This project is a simple yet functional example designed to explore **WebAssembly (WASM) integrated with Rust**. WebAssembly is a binary instruction format that provides a way to run code written in multiple languages on the web at near-native speed. Rust, known for its performance and safety, complements WebAssembly by allowing developers to write fast and secure web applications.

## Project Overview
The project is an educational tool demonstrating how Rust can interact with web technologies through WebAssembly. It features a frontend entirely developed using the **Sycamore framework**, showcasing dynamic interactions and reactivity within a Rust-generated WebAssembly environment.

![WebAssembly project](https://github.com/Jakson-Almeida/Rust-Web-Assembly/blob/main/GUI/images/READ-ME-img1.png)

Additionally, this project utilizes **Tauri** to create a desktop application. Tauri enables the development of a local application for your computer, leveraging Rust for the backend, which enhances performance and security.

## Prerequisites
- Rust and Cargo installed on your system
- [Trunk](https://trunkrs.dev/), a WASM web application bundler for Rust
- [Tauri](https://tauri.studio/), a framework for building tiny, fast binaries for all major desktop platforms

## Installation
1. Clone the repository to your local machine.
2. Ensure that **Rust, Cargo, Trunk, and Tauri** are installed. Trunk can be installed using the following command:

`cargo install trunk`

3. Optionally, to access the application remotely or from other devices within the network, install [ngrok](https://ngrok.com/) on your system.

## Running the Application
Navigate to the project's root directory and execute:

`trunk serve`

![WebAssembly project](https://github.com/Jakson-Almeida/Rust-Web-Assembly/blob/main/GUI/images/READ-ME-img-terminal-trunk-serve.png)

This will compile the project and serve it on `http://[::1]:8080/`. For remote access, start ngrok with:

`ngrok http 8080`

![WebAssembly project](https://github.com/Jakson-Almeida/Rust-Web-Assembly/blob/main/GUI/images/READ-ME-img-terminal-ngrok-http-8080.png)

This provides a URL to access your server from anywhere.

To build and run the desktop application, use Tauri commands:

`cargo tauri build`

This will compile the Tauri application and generate a local executable.

![WebAssembly project](https://github.com/Jakson-Almeida/Rust-Web-Assembly/blob/main/GUI/images/READ-ME-img2.png)

## Conclusion
This setup offers a practical introduction to developing web applications with Rust, WebAssembly, and the Sycamore framework, illustrating the powerful integration capabilities of these technologies. By incorporating Tauri, the project demonstrates how to extend web applications to native desktop applications, further expanding the versatility and reach of Rust-based development.
