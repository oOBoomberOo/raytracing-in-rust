# About

CPU Raytracer written in Rust. Loosely based on the book ["Ray Tracing in One Weekend"](https://raytracing.github.io/) by Peter Shirley.

It currently supports spheres and planes, with either diffuse or reflective materials as configured in `assets/config.yaml`.

# Usage
```bash
cargo run --release
```

With a different configuration file:
```bash
cargo run --release -- -c path/to/config.yaml
```
