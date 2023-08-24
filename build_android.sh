# build static lib for android platform
# if not working run "rustup target add aarch64-linux-android" first
# product can be found under "target/aarch64-linux-android/release"

cargo build --target=aarch64-linux-android --lib --release