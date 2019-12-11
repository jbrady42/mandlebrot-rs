## Mandlebrot Image Generator

### Web Explorer

#### Requirements
- rust
- nodejs

In the explore directory
```
npm run build
```

#### Run server
Run in release mode for extra speed
```
cargo run --release --bin srv
```

Open index.html in browser


### Render frames
```
cargo run --bin mandelbrot -- --help
```

```
mandlebrot 0.1.0
Generate Mandlebrot zoom images

USAGE:
    mandelbrot [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -x, --center-x <center-x>    Set center-x of image [default: 0.001643721971153]
    -y, --center-y <center-y>    Set center-y of image [default: -0.822467633298876]
    -f, --frames <frames>        Set number of frames [default: 1]
    -h, --height <height>        Set height of image [default: 80]
    -s, --scale <start-scale>    Set start scale [default: 0.01]
    -w, --width <width>          Set width of image [default: 80]
```

#### Convert to video

Frames are rendered to `img/`
```
ffmpeg -f image2 -r 15/1 -i img/fractal-%09d.png -c:v libx264 -pix_fmt yuv420p -crf 20 out.mp4
```
