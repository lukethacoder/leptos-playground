<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lukethacoder/word-repo">
    <img src="https://leptos.dev/images/header_logo.svg" alt="Logo" width="220">
  </a>

<h3 align="center">leptos-playground</h3>
  <p align="center">
    playing around with <a href="https://leptos.dev/">leptos</a>
  </p>
</div>


## Installation

Install [trunk](https://trunkrs.dev/) (if you havn't already)

```
cargo install trunk
```

Set rust to the nightly toolchain

Just for this project:

```
rustup toolchain install nightly
rustup override set nightly
```

For every rust project:
```
rustup toolchain install nightly
rustup default nightly
```

Make sure you've added the `wasm32-unknown-unknown` target so that Rust can compile your code to WebAssembly to run in the browser.

```
rustup target add wasm32-unknown-unknown
```

> Make sure you have TailwindCSS installed. You can manually run the `npx tailwindcss -i input.css -o style/output.css` command and it will ask if you want to install.

## Development

Run the development server with:

```
trunk serve --open
```

## Non-windows Machine Development

To get this up and running on a non-windows machine, edit the [Trunk.toml](./Trunk.toml) file:

<!-- ```suggestion
``` -->

```diff
[[hooks]]
stage = "pre_build"
- command = "powershell"
+ command = "sh"
command_arguments = ["-c", "npx tailwindcss -i input.css -o style/output.css"]
```

## Deploy to Vercel

Make sure you have the Vercel CLI installed

```
pnpm i -g vercel
```

Push build to vercel

> NOTE: this assumes you've already run `trunk build`

```
vercel --prod
```
