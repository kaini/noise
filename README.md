# noise [![Build Status](https://travis-ci.org/kaini/noise.png)](https://travis-ci.org/kaini/noise)

## [Library Documentation](http://www.rust-ci.org/kaini/noise/doc/noise/) - [Code Examples](https://github.com/kaini/noise/tree/master/examples)

## Examples

### Two Dimensional Noise

![Smooth 2D Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/noise_2d.png)
![Perlin 2D Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/perlin_2d_colors.png)
![Perlin Generated Map](https://raw.githubusercontent.com/kaini/noise/master/doc/perlin_2d_map.png)
![White Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/white_noise.png)

### One Dimensional Noise

![Perlin 1D Noise](https://raw.githubusercontent.com/kaini/noise/master/doc/perlin_1d.png)
![Interpolation Strategies](https://raw.githubusercontent.com/kaini/noise/master/doc/interpolate.png)

## Features

* White noise
* 1D interpolated noise
* 2D interpolated noise
* Various transformations (including sum, combine, output operations, input operations)
	* Perlin Noise (1D and 2D)

Additional noise generators can be implemented by implementing the `Noise` trait or by using the provided building blocks to modify existing noise.

Smooth noise can be generated with different interpolation strategies: By default Perlin interpolation, cosine interpolation and linear interpolation are provided, but more can be implemented easily by implementing the `Interpolator` trait.

## Todo

* 3D and 4D noise
* Runtime nD Noise
* Generalize everything for all `Float` types
* Remove `Box<Noise<...>>` Types once Rust has the features to do so.
