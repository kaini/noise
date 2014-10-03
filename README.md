# noise [![Build Status](https://travis-ci.org/kaini/noise.png)](https://travis-ci.org/kaini/noise)

## [Library Documentation](http://www.rust-ci.org/kaini/noise/doc/noise/) - [Code Examples](https://github.com/kaini/noise/tree/master/examples)

## Examples

### Two Dimensional Smooth Noise

Amplitude 76.5 with 0 beeing RGB (0, 0, 127.5); Frequency 0.02
![Smooth 2D Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/2d_smooth.png)

### Two Dimensional Coherent Noise

Amplitude 76.5 with 0 beeing RGB (0, 0, 127.5); Frequency 0.02; 6 octaves
![Coherent 2D Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/2d_perlin.png)

### One Dimensional Coherent Noise

Amplitude 60; Frequency 0.01; 6 octaves
![Coherent 1D Noise](https://github.com/kaini/noise/blob/master/doc/1d_perlin.png)

## Features

* 1D random float noise (`Noise<i32, f64>`)
* 1D smooth noise (`Noise<f64, f64>`)
* 2D smooth noise (`Noise<(f64, f64), f64>`)
* Freely configurable "sum" noise resulting in Perlin noise or any other noise pattern you want to build (works with every noise source and all dimensions)

Additional noise generators can be implemented by implementing the `Noise` trait.

Smooth noise can be generated with different interpolation strategies: By default Perlin interpolation, cosine interpolation and linear interpolation are provided, but more can be implemented easily by implementing the `Interpolator` trait.

## Todo

* 3D and 4D noise
* Generalize smooth noise for all `Float` types
