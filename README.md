# Accountability-Explainability Architecture with LLM Agents
This repository contains the code for an accountability and explainability architecture developed for ROS 2-based robots. It leverages Kafka as an event streaming platform to ensure the reliable recording of accountable data, which serves as the foundation for an Agentic RAG-based system that generates natural language explanations.

The experimental environment integrates a ROS 2 and Gazebo-based robotic simulation with Apache Kafka, MongoDB Atlas, and an agentic RAG-based Explainer. ROS 2 messages generated during the robot's operation are processed and transmitted through Kafka, stored in MongoDB Atlas, and subsequently retrieved by the Explainer to generate context-aware explanations.

The repository provides the required components and configuration instructions to deploy the complete experimental environment and reproduce the experimental workflow.

# Software artifacts
ROS 2 Humble

RB1 simulator for ROS 2

[AWS RoboMaker Hospital](https://github.com/aws-robotics/aws-robomaker-hospital-world) World ROS package available [here](https://github.com/jmguerreroh/aws-robomaker-hospital-world/tree/ros2).

The following images show the floor plan of the scenario used in the development of this work, including in the second one an obstacle in the route initially calculated to reach the goal destination. This obstacle will force the change of the pre-computed path.

![imagen](https://user-images.githubusercontent.com/13176052/227868761-7df42f3d-9043-4b07-af27-2b843806be0e.png)

![imagen](https://user-images.githubusercontent.com/13176052/227868841-21b6f0e0-1017-4136-94aa-396ba1205a6b.png)
![Screenshot from 2024-02-19 17-45-53](https://github.com/laurafbec/immutable_explainable_BBR/assets/13176052/d2e9f52a-9366-4bf8-b0d1-4b439fc1947b)
Docker (v29.7.2)

Docker Compose (v5.5.0)

Zookeeper (v7.6.1)

Kafka-connect (v7.6.1)

Kafka (v7.6.1)

langchain v0.3.19

langgraph v0.2.66

langchain-openai v 0.3.7

ragas v0.2.12

Additional requirements and configuration steps for each component are described in their corresponding sections.


## Repository structure

The repository is organized according to the order required to configure and deploy the experimental environment:

```text
agentic-rag-xar/
├── mongodb/
├── kafka-docker/
├── explainer-docker/
├── ros2-gazebo-docker/
└── README.md
```
mongodb/ contains the configuration and instructions required to set up the MongoDB Atlas components used by the system.
kafka-docker/ contains the Kafka infrastructure required for real-time ROS 2 message streaming.
explainer-dokcer/ contains the containerized Explainer implementing the agentic RAG architecture.
[ros2-gazebo-docker/](https://github.com/laurafbec/agentic-rag-xar/tree/main/ros2-gazebo-docker) contains the Docker environment for ROS 2 Humble, Gazebo, Nav2, and the robotic simulation.

## Deployment workflow

Follow the component-specific instructions in the following order:

1. [Configure MongoDB Atlas](./mongodb/README.md)
2. [Deploy Kafka](./kafka-docker/README.md)
3. [Deploy the Explainer](./explainer-docker/README.md)
4. [Set up the ROS 2 and Gazebo environment](./ros2-gazebo-docker/README-ros2-docker.md)
5. Run the complete experiment
