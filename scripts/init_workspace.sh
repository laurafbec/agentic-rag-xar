#!/bin/bash
set -e

# Setup ROS 2 environment
source "/opt/ros/${ROS_DISTRO}/setup.bash"

WS=/home/dockeruser/dev_ws

mkdir -p "${WS}/src"
cd "${WS}"

echo "[INIT] Installing ROS 2 dependencies..."

rosdep install \
    --from-paths src \
    --ignore-src \
    -r \
    -y

if [ ! -f "${WS}/install/setup.bash" ]; then
    echo "[INIT] Workspace has not been built. Building..."

    colcon build \
        --symlink-install \
        --parallel-workers 1

    echo "[INIT] Workspace built successfully."
else
    echo "[INIT] Workspace already built. Skipping build."
fi

