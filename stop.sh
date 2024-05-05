#!/bin/bash
# STOP.sh
#   by Lut99
# 
# Stops the `rust-proxy` server container.
# 
# Unlike `start.sh`, this is exactly equivalent to running `docker compose down`.
# 


# Always run from the script directory
cd "$(dirname "$0")"

echo " > Stopping compose..."
docker compose -f ./docker-compose.yml -f "$compose" down || exit "$?"
