set -euxo pipefail

main() {

    arm-none-eabi-gcc --version

    case $TARGET in
        armebv7r-none-eabi*)
            if [ $TRAVIS_RUST_VERSION = nightly ]; then
                cargo build --target $TARGET
                cargo build --target $TARGET --release
            fi
            ;;

        x86_64-unknown-linux-gnu)
            ;;
    esac
}

main
