# Angry / Rust

An original slingshot-game project with a **Rust + WebAssembly website** that can be hosted on GitHub Pages. The project also keeps an experimental native desktop prototype at the repository root.

## Website

The website lives in `web/` and is written in Rust with Yew. Trunk compiles it to WebAssembly, and GitHub Actions builds and deploys it to GitHub Pages whenever code is pushed to `main`.

Once GitHub Pages is enabled for this repository, the website URL will be:

**https://stellerio.github.io/angry-birds/**

### Enable GitHub Pages once

1. Open the repository's **Settings → Pages**.
2. Under **Build and deployment → Source**, select **GitHub Actions**.
3. Open the **Actions** tab and watch the **Build and deploy Rust website** workflow.
4. When the deployment finishes successfully, visit the website URL above.

### Run the website locally

Install stable Rust from https://rustup.rs and install Trunk:

```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
cd web
trunk serve
```

Trunk will print a local development URL. Open it in your browser; changes to the Rust and CSS files will rebuild automatically.

### How it works

- **Rust + Yew** — interactive page logic and UI.
- **WebAssembly** — runs the compiled Rust in the browser.
- **HTML + CSS** — page entry point and styling.
- **GitHub Actions + GitHub Pages** — build and host the static site automatically.

The interactive playground currently demonstrates the site state and launch counter. It is a website demo, not yet a complete physics game.

## Native prototype

The original desktop prototype remains at the repository root. To try it, run:

```sh
cargo run --release
```

It requires a desktop environment and a graphics backend supported by wgpu.

## License

MIT. The project uses original code and placeholder artwork and does not include Angry Birds' proprietary source code or assets.
