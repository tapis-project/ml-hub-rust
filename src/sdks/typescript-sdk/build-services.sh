for service in gen/*/ ; do
    pushd .
    cd $service
    npm install
    npm run build
    popd
done