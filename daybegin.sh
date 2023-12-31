# Set the default root_dir
root_dir="$HOME/.daybegin"

# Check if config.toml exists
if [ -f "$root_dir/config.toml" ]; then
    # Read the root_dir value from config.toml and expand the tilde
    root_dir=$(toml get "$root_dir/config.toml" root_dir | tr -d '"' | sed "s#^~#$HOME#")
fi

# Read the desired directory from config.toml
desired_dir=$(sed -n -e 's/^work_dir *= *"\(.*\)"/\1/p' "$root_dir/config.toml" 2>/dev/null)

# Expand the tilde to the user's home directory
desired_dir=${desired_dir/#\~/$HOME}

# Change to the desired directory
cd "$desired_dir" || { echo "Failed to change directory: $desired_dir"; exit 1; }

# Get the manifest path
manifest_path="$root_dir/daybegin/Cargo.toml"

echo "manifest_path: $manifest_path"

# Check if the program is already built
program_path="$root_dir/daybegin/target/release/daybegin"
if [ ! -f "$program_path" ]; then
    echo "Building the Rust program..."
    cargo build --manifest-path "$manifest_path" --release
fi

# Run the built program
"$program_path"
