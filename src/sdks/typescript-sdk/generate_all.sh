for service_dir in services/* ; do
    service=`echo $service_dir | cut -d'/' -f 2`
    ./generate.sh $service
done