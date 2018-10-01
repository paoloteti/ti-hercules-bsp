set -euxo pipefail

main() {
    case $TARGET in
        arm*v7r-none-eabi*)
            rustup target add $TARGET
            ;;
    esac
}

main
