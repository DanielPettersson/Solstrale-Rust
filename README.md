[![license](https://img.shields.io/github/license/DanielPettersson/solstrale.svg)](https://tldrlegal.com/license/gnu-general-public-license-v3-(gpl-3))
[![CI](https://github.com/DanielPettersson/solstrale-rust/workflows/CI/badge.svg)](https://github.com/DanielPettersson/solstrale-rust/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/d/solstrale?color=green&label=crates.io)](https://crates.io/crates/solstrale)
[![docs.rs](https://img.shields.io/docsrs/solstrale)](https://docs.rs/solstrale)
------
# Solstrale
A multi-threaded Monte Carlo path tracing library, that as such has features like:
* Global illumination
* Caustics
* Reflection
* Refraction
* Soft shadows

Additionally the library has:
* Loading of obj models with included materials
* Multi-threaded Bvh creation to greatly speed up rendering
* Post processing of rendered images using [Open Image Denoise](https://www.openimagedenoise.org/)

## Credits
The ray tracing is inspired by the excellent [Ray Tracing in One Weekend Book Series](https://github.com/RayTracing/raytracing.github.io) by Peter Shirley
