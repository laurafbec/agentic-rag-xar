# https://hub.docker.com/_/ros
#FROM ros:humble

# https://github.com/osrf/docker_images/blob/master/README.md#ros---1
# https://github.com/osrf/docker_images/blob/master/ros/humble/ubuntu/focal/desktop/Dockerfile
# ROS 2 Humble Desktop
# Pinned to digest for reproducible experiments
FROM osrf/ros:humble-desktop@sha256:b624d8bcea33796d32e0dbd85326722188485759bbd6b538e4785750a5c88a7b

ARG user_id=1000
ARG ros_ws=/home/dockeruser/dev_ws

ENV DEBIAN_FRONTEND=noninteractive

# Install system packages
RUN apt-get update && apt-get install -y sudo \
    ros-${ROS_DISTRO}-gazebo-* \
    python-is-python3 \
    python3-rosdep \
    nano \
    less \
    xterm
    #&& rm -rf /var/lib/apt/lists/*

# Create non-root user with passwordless sudo
RUN useradd -m --uid ${user_id} dockeruser \
    && echo "dockeruser ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/dockeruser \
    && chmod 0440 /etc/sudoers.d/dockeruser
USER dockeruser

# Working directory
WORKDIR /home/dockeruser

# Environment variables
ENV HOME=/home/dockeruser
ENV PATH="/home/dockeruser/.local/bin:${PATH}"

# Force Python stdout and stderr streams to be unbuffered.
ENV PYTHONUNBUFFERED 1

# Configure ROS 2, Gazebo and workspace environment
RUN echo "source /opt/ros/${ROS_DISTRO}/setup.bash" >> ~/.bashrc \
    && echo "source /usr/share/gazebo/setup.bash" >> ~/.bashrc \
    && echo "if [ -f /home/dockeruser/dev_ws/install/setup.bash ]; then source /home/dockeruser/dev_ws/install/setup.bash; fi" >> ~/.bashrc

# Configure colcon_cd
# https://docs.ros.org/en/humble/Tutorials/Configuring-ROS2-Environment.html#add-colcon-cd-to-your-shell-startup-script
RUN echo "source /usr/share/colcon_cd/function/colcon_cd.sh" >> ~/.bashrc \
    && echo "export _colcon_cd_root=${ros_ws}" >> ~/.bashrc

# Entrypoint
ENTRYPOINT ["/home/dockeruser/scripts/entrypoint.sh"]

CMD ["sleep", "infinity"]
