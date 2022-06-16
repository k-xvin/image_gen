generate name:
    @echo 'Generating template {{name}}...'
    cp template.rs src/{{name}}.rs
    sed -i 's/template/{{name}}/' src/{{name}}.rs
    echo >> Cargo.toml
    sed 's/template/{{name}}/' template.txt >> Cargo.toml
    echo >> Cargo.toml
    @echo 'Template for {{name}} generated!'

remove name:
    @echo 'Removing {{name}}...'
    rm -i src/{{name}}.rs
    line=$(grep -n 'name = "{{name}}"' Cargo.toml | awk -F  ":" '{print $1}') && \
    range_low=$((line-2)) && \
    range_high=$((line+1))d && \
    sed -i "$range_low,$range_high" Cargo.toml
    @echo '{{name}} removed!'

run name:
    @echo 'Running {{name}}...'
    cargo run --release --example {{name}}

gif name:
    @echo 'Generating gif for {{name}}...'
    # TODO ffmpeg stuff here
    @echo 'Gif for {{name}} generated!'