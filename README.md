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
* Bump mapping
* Light attenuation

## Example output
![happy](https://github.com/DanielPettersson/solstrale-rust/assets/3603911/c5357792-a3dc-42f9-8230-320140f9c30e)
![sponza-bump2](https://github.com/DanielPettersson/solstrale-rust/assets/3603911/0ab79ed9-cddf-46b1-84e7-03cef35f5600)

## Credits
The ray tracing is inspired by the excellent [Ray Tracing in One Weekend Book Series](https://github.com/RayTracing/raytracing.github.io) by Peter Shirley
