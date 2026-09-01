# ROS 2 Docker Environment

Docker environment for reproducing the ROS 2 and Gazebo experiments.

The environment provides a containerized ROS 2 Humble setup with Gazebo, Nav2, and the required dependencies for the experimental ROS 2 workspace.
## System Requirements
The Docker environment has been tested and validated on Ubuntu 22.04 LTS.

Ubuntu 22.04 LTS is required as the host operating system to reproduce the experiments described in this repository. Other host operating systems or Ubuntu versions have not been validated and may present compatibility issues with the ROS 2 and Gazebo components.

## Script permissions
Before building or starting the environment, make sure that the shell scripts in the [scripts](https://github.com/laurafbec/agentic-rag-xar/tree/main/ros2-gazebo-docker/scripts) directory have execution permissions:
```bash
chmod +x scripts/*.sh
```
The scripts are used to build and initialize the ROS 2 environment.

## Getting started

Build the Docker image:

```bash
cd agentic-rag-xar/ros2-gazebo-docker

docker compose build
docker compose up
```
If your user id is not 1000, add ``` --build-arg user_id=$(id -u)``` to docker compose build command.

## First startup

During the first startup, the ROS 2 workspace is initialized automatically. This includes installing the required dependencies with `rosdep` and building the workspace with `colcon`.

The first startup may take several minutes. **Wait until the initialization process finishes before running the experiment.**

A successful first initialization ends with:

```text
[INIT] Workspace built successfully.
```
On subsequent container starts, if the workspace has already been initialized, the build is skipped and the following message is displayed:
```text
[INIT] Workspace already built. Skipping build.
[ENTRYPOINT] Workspace ready.
```
**Wait for one of these messages before proceeding.** This ensures that the workspace initialization and compilation have completed successfully.
### Running the hospital scenario
Once the Docker container is running and the workspace initialization has completed, open a shell in the container:
```bash
docker compose exec ros2 bash
```
The ROS 2 and workspace environments are sourced automatically.
To launch the [AWS RoboMaker hospital scenario](https://github.com/aws-robotics/aws-robomaker-hospital-world/tree/ros2), execute:
```bash
ros2 launch rb1_sandbox hospital_door_fullmap.launch.py
```
This launches the hospital environment in Gazebo together with the RB-1 robot and the navigation components required for the experiment.

The launch process starts Gazebo, loads the hospital world and map, and initializes the robot navigation stack.

Wait until the Gazebo window and the required ROS 2 nodes have started before interacting with the scenario.

### Running the message processing and Kafka producer
The `rt_recorder_explainer` package provides the service responsible for processing the ROS 2 messages, establishing causal relationships between them, and publishing the resulting messages to Kafka.
In a second terminal, access the running container:
```bash
docker compose exec ros2 bash
```
Then launch the service:

```bash
ros2 launch rt_recorder_explainer rt_recorder_explainer_srv.launch.py
```
Once the service has been started, the Kafka producer can be enabled by calling in a new terminal from the container:
```bash
ros2 service call /kafka_producer_srv std_srvs/srv/SetBool data:\ true
```
To stop the Kafka producer run:
```bash
ros2 service call /kafka_producer_srv std_srvs/srv/SetBool data:\ false
```
### Running the hospital navigation

The `waypoint_navigation` package provides the navigation component used to autonomously move the robot between three configurable positions in the hospital scenario.

**This step requires an additional terminal. Keep the hospital scenario and the `rt_recorder_explainer` service running in their respective terminals.**

In a new terminal, access the running container:

```bash
docker compose exec ros2 bash
```
First, launch the navigation configuration:
```bash
ros2 launch waypoint_navigation waypoint_navigation_hospital.launch.py
```
In a new terminal from the container, start the navigation node:
```bash
ros2 run waypoint_navigation waypoint_navigation_hospital_node
```
The robot will navigate autonomously to the three positions configured for the experiment.

The three target positions can be configured in the corresponding parameter file before launching the navigation.

### Terminal overview

The complete experiment requires five terminals running simultaneously:

| Terminal | Process |
|---|---|
| 1 | Hospital scenario and Gazebo |
| 2 | `rt_recorder_explainer` service |
| 3 | Kafka producer service activation/deactivation |
| 4 | Hospital navigation launch |
| 5 | Hospital navigation node |

Keep all five terminals running during the experiment, except that the Kafka producer in Terminal 3 is explicitly enabled and disabled using the service calls described above.
## Rebuilding after modifications
If any of the ROS 2 projects in ```dev_ws/src``` are modified, the workspace must be rebuilt before running the experiment.
From a new terminal in the container, execute:

```bash
~./scripts/build.sh
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
