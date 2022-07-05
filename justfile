default:
    @just --list

# Create a new artpiece to work on
@create name:
    echo 'Generating template {{name}}...'
    cp template.rs src/{{name}}.rs
    sed -i 's/template/{{name}}/g' src/{{name}}.rs
    echo >> Cargo.toml
    sed 's/template/{{name}}/g' template.txt >> Cargo.toml
    echo >> Cargo.toml
    echo 'Template for {{name}} generated!'

# Delete a specific artpiece
@remove name:
    echo 'Removing {{name}}...'
    rm -i src/{{name}}.rs
    line=$(grep -n 'name = "{{name}}"' Cargo.toml | awk -F  ":" '{print $1}') && \
    range_low=$((line-2)) && \
    range_high=$((line+1))d && \
    sed -i "$range_low,$range_high" Cargo.toml
    echo '{{name}} removed!'

# Run the artpiece in non-release mode
r name:
    @echo 'Running {{name}}...'
    cargo run --bin {{name}}

# Run the artpiece in release mode
run name:
    @echo 'Running {{name}}...'
    cargo run --release --bin {{name}}

# Convert the artpiece into a gif
gif_full name framerate='30':
    @echo 'Generating gif for {{name}}...'
    ffmpeg -framerate {{framerate}} -i "screenshots/{{name}}/{{name}}_%d.jpg" -filter_complex "fps={{framerate}},split=2[palette_in][gif];[palette_in]palettegen[palette_out];[gif]fifo[gif_fifo]; [gif_fifo][palette_out]paletteuse" -y screenshots/{{name}}_full.gif
    gifsicle -V -b -O3 --lossy=20 screenshots/{{name}}_full.gif
    @echo 'Gif for {{name}} generated!'cd

gif_limit name framerate='30':
    @echo 'Generating gif for {{name}}...'
    # // we cap at 160000000 bytes (16Mb)
    ffmpeg -framerate {{framerate}} -i "screenshots/{{name}}/{{name}}_%d.jpg" -filter_complex "scale=800:600,fps={{framerate}},split=2[palette_in][gif];[palette_in]palettegen[palette_out];[gif]fifo[gif_fifo]; [gif_fifo][palette_out]paletteuse" -y -fs 10000000 screenshots/{{name}}_limit.gif
    @echo  'Running ffmpeg output through gifsicle'
    # // use gifsicle to cut gif size by >50%, so it can be under 8Mb for discord
    gifsicle -V -b -O3 --lossy=35 screenshots/{{name}}_limit.gif
    @echo 'Gif for {{name}} generated!'cd

# gif_no_filter name framerate='1':
#     @echo 'Generating gif for {{name}}...'
#     ffmpeg -framerate {{framerate}} -i "screenshots/{{name}}/{{name}}_%d.png" screenshots/{{name}}_no_filter.gif
#     @echo 'Gif for {{name}} generated!'cd