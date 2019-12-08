## Mandlebrot Image Generator


#### Convert to video
```
ffmpeg -f image2 -r 15/1 -i img/fractal-%09d.png -c:v libx264 -pix_fmt yuv420p -crf 20 out.mp4
```
