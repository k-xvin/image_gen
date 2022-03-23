`cargo run --example square1`

`cargo run --release --example square1`

`ffmpeg -framerate 1 -i "square4/image_%d.png" -filter_complex  "fps=1,split=2[palette_in][gif];[palette_in]palettegen[palette_out];[gif]fifo[gif_fifo]; [gif_fifo][palette_out]paletteuse" -y square4.gif`

// `ffmpeg -framerate 1 -i "square4/image_%d.png"  square4.gif`


