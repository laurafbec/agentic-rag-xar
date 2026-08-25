#!/bin/bash
set -e

source "/opt/ros/${ROS_DISTRO}/setup.bash"

if [ -f ~/dev_ws/install/setup.bash ]; then
    source ~/dev_ws/install/setup.bash
fi

cd ~/dev_ws

colcon build --symlink-install