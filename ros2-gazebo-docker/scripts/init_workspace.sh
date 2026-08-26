#!/bin/bash
set -e

# Setup ROS 2 environment
source "/opt/ros/${ROS_DISTRO}/setup.bash"

WS=/home/dockeruser/dev_ws

mkdir -p "${WS}/src"
cd "${WS}"

# Initialize rosdep if necessary
if [ ! -f /etc/ros/rosdep/sources.list.d/20-default.list ]; then
    echo "[INIT] Initializing rosdep..."
    sudo rosdep init
fi

echo "[INIT] Updating rosdep..."
rosdep update

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

