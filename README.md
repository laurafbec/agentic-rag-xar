## Requirements

- Docker
- Docker Compose

- git clone https://github.com/laurafbec/agentic-rag-xar.git

# ROS 2 Docker Environment

Docker environment for reproducing the ROS 2 and Gazebo experiments associated with the paper.

The environment provides a containerized ROS 2 Humble setup with Gazebo, Nav2, and the required dependencies for the experimental ROS 2 workspace.

## Getting started

Build the Docker image:

```bash
cd agentic-rag-xar/ros2-gazebo-docker

docker compose build
docker compose up
```

```bash
docker compose exec --user root ros2 bash
```
