for service in gen/*/ ; do
    pushd .
    cd $service
    rm -rf node_modules
    rm -rf dist
    popd
done
