#!/usr/bin/env bash
set -euf -o pipefail

sudo apt-get update

sudo apt-get install -y build-essential libpq-dev postgresql-client
cargo install diesel_cli --no-default-features --features postgres
