if [ $ENV == "debug" ]; then
    cargo watch -x "run"
elif [ $ENV == "release" ]; then
    cargo watch -x "run --release"
else
    echo "ENV must be set to debug or release"
fi
