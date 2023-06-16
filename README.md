# All Colors

'Rust' implementation of All Colors
(see the C# branch for more documentation)





## Color Space
An `RGB` colorspace is a 3-dimensional space containing all `RGB` values from `0`-`255` (`u8`)
If all possible colors in this space are used, it is a cube `256x256x256` = `16,777,216`

The distance between two colors is given by the formula:
`dist = sqrt((first.red - second.red) * (first.green - second.green) * (first.blue - second.blue))`
This can also be roughly simplified by omitting the `sqrt`
The furthest dist would thusly be `4,096` (16k, above for non-sqrt)
If we want to translate that to an (x,y) dist, we can translate:
`xy_dist = sqrt((first.x - second.x) * (first.y - second.y))`
`max_xy_dist = sqrt(width * height)`
`translate xy_dist = 4096 * (xy_dist / max_xy_dist)`



## Useful `rust` links:
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
- 