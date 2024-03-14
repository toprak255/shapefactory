
# Shapefactory

A Rust app for creating regular n-gons according to given parameters

## Build

```
https://github.com/toprak255/shapefactory.git

cd shapefactory

cargo build --release
```
## Documentation
After building you can run executable to display all available commands ,output should look like this
```
usage: shapefactory.exe [-corner <corner count>] [-Width <canvas-width>] [-Height <canvas-height>] [-scale <value>] 
                        [-random-scale] [-random-rotation] [-fg <hex-32bit>] [-bg <hex-32bit>] [-f <folderPath>] [-n <filename>] 
                        [-count <number of images>] [-random-color]
```
-fg and -bg isn't working as of now and are just ignored

Every argument other than -corner -count and -filename has default "usable" values and program can work without them being set by user

```
width:1001
height:1001
scale:1.
random_rotation:false,
random_scale:false,
random_color:false,
foreground_color:Rgba([255,255,255,255])
background_color:Rgba([0,0,0,255]),
```
