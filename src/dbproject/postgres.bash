#!/usr/bin/env bash
set -euf -o pipefail

CONTAINER_NAME=dbproject

docker run --rm --name "${CONTAINER_NAME}" \
    --network srv_default \
    -e POSTGRES_PASSWORD=admin \
    -d postgres || true

sleep 5

echo "DATABASE_URL=postgres://postgres:admin@${CONTAINER_NAME}/diesel" > .env

echo "Press \q to quit"
psql -h "${CONTAINER_NAME}" -U postgres