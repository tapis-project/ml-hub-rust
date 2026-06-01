#!/usr/bin/env bash

set -e

# Define your stable hostname based on your StatefulSet and Headless Service
STABLE_HOSTNAME="mlhub-mongo-stateful-set-0.mlhub-mongo-headless-service.tapisdev.svc.cluster.local"
CONNECTION_STRING="mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@localhost:27017/admin"

# Wait for MongoDB to respond
attempt=0
until mongosh "$CONNECTION_STRING" --eval "db.adminCommand({ ping: 1 })" >/dev/null 2>&1; do
    echo "Waiting for MongoDB to start... Attempt $attempt"
    sleep 1
    ((attempt++))
    if [ "$attempt" -eq 60 ]; then
    echo "Mongo failed to start"
    exit 1
    fi
done

# Initialize replica set if not already initialized
RS_STATUS=$(mongosh "$CONNECTION_STRING" --quiet --eval "try { rs.status().ok } catch(e) { 0 }")
if [ "$RS_STATUS" != "1" ]; then
    echo "Initializing replica set..."
    # USE STABLE_HOSTNAME HERE so the RS metadata is correct
    mongosh "$CONNECTION_STRING" --eval "rs.initiate({_id:'rs0', members:[{_id:0, host:'${STABLE_HOSTNAME}:27017'}]})"
    
    until mongosh "$CONNECTION_STRING" --quiet --eval "db.hello().isWritablePrimary" | grep true >/dev/null 2>&1; do
    echo "Waiting for PRIMARY to become writable..."
    sleep 2
    done
    echo "Replica set initialized."
else
    echo "Replica set already initialized."
fi


# Ensure the replica set is configured to use the stable hostname and NOT localhost.
# If set to localhost, this will update it
CURRENT_HOST=$(mongosh "$CONNECTION_STRING" --quiet --eval "try { rs.conf().members[0].host } catch(e) { '' }")
if [[ "$CURRENT_HOST" != "${STABLE_HOSTNAME}:27017" ]]; then
    echo "Reconfiguring replica set..."
    mongosh "$CONNECTION_STRING" --eval "
    cfg = rs.conf();
    cfg.members[0].host = '${STABLE_HOSTNAME}:27017';
    rs.reconfig(cfg, { force: true });
    "
fi

# Initialize the MLHub user
mongosh "$CONNECTION_STRING" --eval "
    var targetDb = db.getSiblingDB('admin');
    if (!targetDb.getUser('${MLHUB_USERNAME}')) {
    targetDb.createUser({
        user: '${MLHUB_USERNAME}',
        pwd: '${MLHUB_PASSWORD}',
        roles: [{ role: 'readWriteAnyDatabase', db: 'admin' }]
    });
    print('User created.');
    } else {
    print('User already exists in mlhub database, skipping.');
    }
"

echo "Initialization complete. Entering sleep to keep sidecar alive."
exec tail -f /dev/null