if [ $ENV == "debug" ]; then
    trunk serve
elif [ $ENV == "release" ]; then
    trunk serve --release
else
    echo "ENV must be set to debug or release"
fi
