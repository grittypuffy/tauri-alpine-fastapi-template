# Tauri + Alpine.js + TypeScript + Bun + FastAPI template

A minimal template for Tauri, with FastAPI sidecar and Alpine.js frontend using `bun` for package management and runtime with Vite for faster builds.

Heavily inspired by [tauri-fastapi-react-app](https://github.com/guilhermeprokisch/tauri-fastapi-react-app/) and created out of need for:

1. Supporting `bun` instead of `npm`(as it rocks)
2. Usage of Alpine.js out of the box for my applications, primarily due to its minimalism
3. Supporting sidecars for FastAPI backend in Tauri v2 (the above template is configured for v1), as it helps going with the latest and greatest

# Prerequisites

Before you begin, ensure you have the following installed:

- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/)
- [Python](https://www.python.org/) (v3.12 or later)
- [Poetry](https://python-poetry.org/)

## Getting Started

1. Clone this repository:

   ```
   git clone https://codeberg.org/grittypuffy/tauri-alpine-fastapi-template.git
   cd tauri-alpine-fastapi-template
   ```

2. Install frontend dependencies:

   ```
   bun install
   ```

3. Install Python dependencies:

   ```
   poetry install
   ```

4. Run the development server:
   ```
   bun run tauri dev
   ```

This will start both the Tauri/Alpine frontend and the FastAPI backend.

# More information

Check out [tauri-fastapi-react-app](https://github.com/guilhermeprokisch/tauri-fastapi-react-app/)'s README for more information regarding the project configuration.