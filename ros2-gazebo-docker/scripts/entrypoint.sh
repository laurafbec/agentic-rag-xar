#!/bin/bash
set -e

# Setup ROS 2 environment
source "/opt/ros/${ROS_DISTRO}/setup.bash"

echo "[ENTRYPOINT] Checking ROS 2 workspace..."

~/scripts/init_workspace.sh

echo "[ENTRYPOINT] Workspace ready."

exec "$@"