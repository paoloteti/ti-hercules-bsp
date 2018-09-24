set -euxo pipefail

main() {
    case $TARGET in
        armebv7r-none-eabi*)
            rustup target add $TARGET
            ;;
    esac
}

main
