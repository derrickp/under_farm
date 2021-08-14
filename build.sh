cargo build --release
mkdir -p build
cp -R ./assets ./build
cp ./target/release/under_farm ./build
zip -r under_farm.zip ./build
