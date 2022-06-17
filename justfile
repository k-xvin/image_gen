default:
    @just --list

# Create a new artpiece to work on
@generate name:
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
    cargo run --example {{name}}

# Run the artpiece in release mode
run name:
    @echo 'Running {{name}}...'
    cargo run --release --example {{name}}

# Convert the artpiece into a gif
gif name framerate='1':
    @echo 'Generating gif for {{name}}...'
    ffmpeg -framerate {{framerate}} -i "screenshots/{{name}}/{{name}}_%d.png" -filter_complex "fps={{framerate}},split=2[palette_in][gif];[palette_in]palettegen[palette_out];[gif]fifo[gif_fifo]; [gif_fifo][palette_out]paletteuse" -y screenshots/{{name}}.gif
    # ffmpeg -framerate {{framerate}} -i "screenshots/{{name}}/{{name}}_%d.png" screenshots/{{name}}.gif
    @echo 'Gif for {{name}} generated!'cd