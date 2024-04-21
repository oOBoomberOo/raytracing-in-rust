# About

CPU Raytracer written in Rust. Loosely based on the book ["Ray Tracing in One Weekend"](https://raytracing.github.io/) by Peter Shirley.

It currently supports spheres and planes, with either diffuse or reflective materials as configured in `assets/config.yaml`.

# Usage
```bash
cargo run --release
```
Or with a different configuration file:
```bash
cargo run --release -- -c path/to/config.yaml
```

The camera also can be control using WASD keys.

![Screenshot 1](https://github.com/oOBoomberOo/raytracing-in-rust/blob/main/screenshots/1.png?raw=true)
![Screenshot 2](https://github.com/oOBoomberOo/raytracing-in-rust/blob/main/screenshots/2.png?raw=true)
![Screenshot 3](https://github.com/oOBoomberOo/raytracing-in-rust/blob/main/screenshots/3.png?raw=true)
