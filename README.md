# Ember

A ray tracer built from scratch in Rust, following [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![Ember output — normal-mapped sphere](ember.png)

## About

Ember is a minimal, dependency-free ray tracer that renders scenes to PPM image files. The goal is to understand every layer of the rendering pipeline by building it by hand - no graphics libraries, no shortcuts.

### Current Features

- Custom `Vec3` math library with operator overloading (`+`, `-`, `*`, `/`, negation)
- Ray-sphere intersection using the quadratic discriminant
- Surface normal visualization (RGB-mapped normals)
- Linear gradient sky background
- PPM image output at 1920×1080

### Sample Output

The image above shows a unit sphere at the origin with its surface normals mapped to RGB color space - green/cyan at the top, magenta/purple at the bottom, blue facing the camera. The background is a vertical lerp between white and sky blue.

## Project Structure

```
src/
├── main.rs      # Scene setup, render loop, image output
├── vec3.rs      # Vec3/Point3 type with arithmetic ops and utility functions
├── ray.rs       # Ray type with origin + direction and parametric evaluation
└── color.rs     # Color type alias and PPM color writing
```

## Build & Run

```bash
cargo build --release
cargo run --release
```

This produces `ember.ppm` in the project root. Convert to PNG with:

```bash
# ImageMagick
magick ember.ppm ember.png

# or ffmpeg
ffmpeg -i ember.ppm ember.png
```

## Roadmap

- [ ] Multiple spheres / object list
- [ ] Antialiasing (multi-sample per pixel)
- [ ] Diffuse materials (Lambertian)
- [ ] Metal and dielectric materials
- [ ] Positionable camera
- [ ] Defocus blur (depth of field)
- [ ] BVH acceleration

## References

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley
- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

## License

MIT
