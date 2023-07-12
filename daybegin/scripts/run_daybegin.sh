

# Read the root_dir value from config.toml and expand the tilde
root_dir=$(toml get $HOME/.daybegin/config.toml root_dir | tr -d '"' | sed "s#^~#$HOME#")

# Read the desired directory from config.toml
desired_dir=$(sed -n -e 's/^work_dir = "\(.*\)"/\1/p' ~/.daybegin/config.toml)

# Expand the tilde to the user's home directory
desired_dir=${desired_dir/#\~/$HOME}

# Change to the desired directory
cd "$desired_dir" || { echo "Failed to change directory: $desired_dir"; exit 1; }

# Get the manifest path
manifest_path="$root_dir/daybegin/Cargo.toml"

echo "manifest_path: $manifest_path"

# Run the Rust program
cargo run --manifest-path $manifest_path --bin daybegin