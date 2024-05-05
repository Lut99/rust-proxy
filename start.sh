#!/bin/bash
# START.sh
#   by Lut99
# 
# Starts the `rust-proxy` server container.
# 
# This is almost equivalent to running `docker compose up`, except:
# - certificate file symlinks are followed before mounting them.
# 

# The temporary docker compose file to create
compose="./.docker-compose.yml.local"

# The list of domains for which to add certificates
domains=("dehoek-studio.nl" "dev.dehoek-studio.nl" "dnd.timinc.nl" "server.timinc.nl")


# Always run from the script directory
cd "$(dirname "$0")"

# Obtain the user IDs
uid=$(id -u)
gid=$(id -g)
echo " > User ID / Group ID: $uid/$gid"

# Obtain certificate paths
echo " > Obtaining certificate fullpaths..."
printf "services:\n  rust-proxy:\n    volumes:\n" > "$compose"
for domain in ${domains[@]}; do
    # Read the live symbolic links
    cert=("$(sudo readlink "/etc/letsencrypt/live/$domain/fullchain.pem")")
    key=("$(sudo readlink "/etc/letsencrypt/live/$domain/privkey.pem")")

    # Get their fullpath equivalents
    cert=("$(sudo realpath "/etc/letsencrypt/live/$domain/$cert")")
    key=("$(sudo realpath "/etc/letsencrypt/live/$domain/$key")")

    # Write that to the Dockerfile as volumes
    echo "    - $cert:/certs/$domain/fullchain.pem:ro" >> "$compose"
    echo "    - $key:/certs/$domain/privkey.pem:ro" >> "$compose"
done

# Now start the container
echo " > Starting compose..."
docker compose -f ./docker-compose.yml -f "$compose" up -d --build || exit "$?"
