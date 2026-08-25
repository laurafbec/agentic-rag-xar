## Requirements

- Docker
- Docker Compose

- git clone https://github.com/laurafbec/agentic-rag-xar.git

# ROS 2 Docker Environment

Docker environment for reproducing the ROS 2 and Gazebo experiments.

The environment provides a containerized ROS 2 Humble setup with Gazebo, Nav2, and the required dependencies for the experimental ROS 2 workspace.

## Getting started

Build the Docker image:

```bash
cd agentic-rag-xar/ros2-gazebo-docker

docker compose build
docker compose up
```
If your user id is not 1000, add ``` --build-arg user_id=$(id -u)``` to docker compose build command.

### First startup

During the first startup, the ROS 2 workspace is initialized automatically. This includes installing the required dependencies with `rosdep` and building the workspace with `colcon`.

The first startup may take several minutes. **Wait until the initialization process finishes before running the experiment.**

A successful first initialization ends with:

```text
[INIT] Workspace built successfully.
```
On subsequent container starts, if the workspace has already been initialized, the build is skipped and the following message is displayed:
```text
[INIT] Workspace already built. Skipping build.
```

## Other commands

The following commands can be used to manage and access the Docker environment.

### Stop the container

To stop the running container without removing it:

```bash
docker compose stop
```
The container and its associated Docker resources are preserved. It can be started again with:
```bash
docker compose start
```
### Remove the container
To stop and remove the container:
```bash
docker compose down
```

```bash
docker compose exec ros2 bash
```
