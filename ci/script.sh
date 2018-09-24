set -euxo pipefail

main() {

    arm-none-eabi-gcc --version

    case $TARGET in
        armebv7r-none-eabi*)
            if [ $TRAVIS_RUST_VERSION = nightly ]; then
                cargo check --target $TARGET
                cargo check --target $TARGET --release
            fi
            ;;

        x86_64-unknown-linux-gnu)
            ;;
    esac
}

main
