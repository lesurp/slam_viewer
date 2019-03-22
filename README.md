# SLAM VIEWER

View your cameras + points 3d + rays easily.

# Screenshot

![split_0](https://user-images.githubusercontent.com/12513150/51759633-f9529e00-20c8-11e9-9046-d9dbfd77ed23.png)

# Explanation

We load a file containing:

1. Camera poses
2. Pixel coordinates
3. 3d points

We display the camera and the points exactly as described, and a ray from the camera centers to infinity and beyond.

# Settings

1. Intrinsics K (** FOR NOW HARDCODED WITHIN THE SOURCE :<**) [default = I<sub>3</sub>]
2. The scale for the rendering (passed through argv[1] as a float) [default  = 1]
3. The path to the data file (passed through argv[2]) [default = 'data']


There's another setting for the ray length that needs to be made public.

# Data format

## For all inputs

1. Data is loaded by default from a file called `data` , or from argv[2] if it's defined
2. Values should be floats
3. Whitespace separated (\\t or spaces etc.)
4. unless specified, any other character will invalidate the line (ie ignore it)

Note that 2 & 3 correspond to the default format output by Eigen when calling `cout <<`.

## Specifics

### Camera poses

1. Must be in the [ R\_wc | t\_wc ] form (3 * 4 matrix) **NOT THE OPPOSITE**
2. The three lines should be consecutive

### Points 3d

1. All three dimensions should be on the same line (x, y, z), ie: `1.0 1.0 1.0`

### Pixels

1. Must *immediatly* follow a camera pose or another pixel (we assume the pixels belong to the last camera pose defined)
2. The two coordinates should be on the same line (xp, yp), ie: `1.0 1.0`
3. The format `[1.0, 1.0]` is also accepted (yes, that's the format output by `cout << cv::Point2f`)

# Compile in release!

Rust is very slow in debug mode, don't forget that!
