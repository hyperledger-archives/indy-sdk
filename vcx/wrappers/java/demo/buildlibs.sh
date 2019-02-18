current_dir=$(pwd)
source ~/.profile
cd ..
./gradlew build
cd ../../libvcx
cargo build
cd ../../libindy
cargo build
cd ../libnullpay
cargo build

echo $current_dir
cd $current_dir

if [[ "$OSTYPE" == "linux-gnu" ]]; then
        sudo cp ../../../libvcx/target/debug/libvcx.so /usr/lib/libvcx.so
        sudo cp ../../../../libindy/target/debug/libindy.so /usr/lib/libindy.so
        sudo cp ../../../../libnullpay/target/debug/libnullpay.so /usr/lib/libnullpay.so

elif [[ "$OSTYPE" == "darwin"* ]]; then
        # Mac OSX
        cp ../../../libvcx/target/debug/libvcx.dylib /usr/local/lib/libvcx.dylib
        cp ../../../../libindy/target/debug/libindy.dylib /usr/local/lib/libindy.dylib
        cp ../../../../libnullpay/target/debug/libnullpay.dylib /usr/local/lib/libnullpay.dylib
fi