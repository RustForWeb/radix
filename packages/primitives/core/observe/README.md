<p align="center">
    <a href="../../../../logo.svg">
        <img src="../../../../logo.svg" width="300" height="200" alt="Rust Radix Logo">
    </a>
</p>

<h1 align="center">radix-rect</h1>

This is an internal utility, not intended for public usage.

[Rust Radix](https://github.com/RustForWeb/radix) is a Rust port of [Radix](https://www.radix-ui.com/primitives).

## Documentation

See [the Rust Radix book](https://radix.rustforweb.org/) for documentation.

## Rust for Web

The Rust Radix project is part of [Rust for Web](https://github.com/RustForWeb).

[Rust for Web](https://github.com/RustForWeb) creates and ports web UI libraries for Rust. All projects are free and open source.

## Modifications

This implementation is generalized where the callbacks are provided with the `ResizeObserverEntry`
directly so any dependants that need to observe changes can use it.
