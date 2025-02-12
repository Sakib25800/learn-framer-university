#!/bin/sh

# Set the Rust backtrace for debugging
export RUST_BACKTRACE=1

# If the backend is started before postgres is ready, the migrations will fail
until diesel migration run --locked-schema; do
  echo "Migrations failed, retrying in 5 seconds..."
  sleep 5
done

cargo run
