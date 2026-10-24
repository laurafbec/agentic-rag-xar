# Accountability-Explainability Architecture with LLM Agents
This repository contains the code and software artifacts for an accountability and explainability architecture developed for ROS 2-based robots. The architecture leverages Apache Kafka as an event streaming platform to support the reliable recording of accountable data, which serves as the foundation for an Agentic RAG-based system that generates natural language explanations.

The experimental environment integrates a ROS 2 and Gazebo-based robotic simulation with Apache Kafka, MongoDB Atlas, and an Agentic RAG-based Explainer. ROS 2 messages generated during the robot's operation are processed and transmitted through Kafka, stored in MongoDB Atlas, and subsequently retrieved by the Explainer to generate context-aware explanations.

The repository provides the required software components, configuration files, and instructions to deploy the complete experimental environment and reproduce the experimental workflow.

## Requirements
- Docker and Docker Compose
- A MongoDB Atlas account (M0 free-tier is sufficient)
- API credentials for OpenAI and OpenRouter (or compatible alternative providers, see `explainer-docker/README.md`)

## Software artifacts
The main software components and versions used in the experimental environment are:

| Component | Version |
|---|---|
| ROS 2 | Humble |
| RB-1 simulator | ROS 2 |
| Docker | 29.7.2 |
| Docker Compose | 5.5.0 |
| Zookeeper | 7.6.1 |
| Kafka | 7.6.1 |
| Kafka Connect | 7.6.1 |
| LangChain | 0.3.19 |
| LangGraph | 0.2.66 |
| LangChain OpenAI | 0.3.7 |
| RAGAS | 0.2.12 |

The AWS RoboMaker Hospital World is used as the simulation environment. The ROS 2 version of the world is available [here](https://github.com/aws-robotics/aws-robomaker-hospital-world/tree/ros2).

## Configuration

The `broker1` hostname must resolve to the IP address of the Kafka server from the host where the accountable service is launched.

## Repository structure

The repository is organized according to the order required to configure and deploy the experimental environment:

```text
agentic-rag-xar/
├── mongodb-atlas-vector-search/
├── accountability-kafka-docker/
├── explainer-docker/
├── ros2-gazebo-docker/
└── README.md
```
[mongodb-atlas-vector-search/](./mongodb-atlas-vector-search) contains the configuration and instructions required to set up the MongoDB Atlas components used by the system.

[accountability-kafka-docker/](./accountability-kafka-docker) contains the Kafka infrastructure required for real-time ROS 2 message streaming.

[explainer-docker/](./explainer-docker) contains the containerized Explainer implementing the agentic RAG architecture.

[ros2-gazebo-docker/](./ros2-gazebo-docker) contains the Docker environment for ROS 2 Humble, Gazebo, Nav2, and the robotic simulation.

## Deployment workflow
To reproduce the experimental environment, follow the instructions provided in each component's README in the order listed below:

1. [Configure MongoDB Atlas Vector Search](./mongodb-atlas-vector-search/README.md)
2. [Deploy Kafka](./accountability-kafka-docker/README.md)
3. [Deploy the Explainer](./explainer-docker/README.md)
4. [Set up the ROS 2 and Gazebo environment](./ros2-gazebo-docker/README.md)
5. Run the complete experiment

The Kafka infrastructure and the Explainer must be available before starting the autonomous navigation experiment. MongoDB Atlas must be configured beforehand so that the messages generated during the experiment can be stored and retrieved by the Explainer.

## Architecture and data flow

The complete accountability and explainability architecture integrates ROS 2, Apache Kafka, MongoDB Atlas, and an Agentic RAG workflow:

```text
ROS 2 robot
    |
    v
Apache Kafka
    |
    v
Accountable robot data
    |
    v
MongoDB Atlas
    |
    |  Document insertion
    v
Atlas Trigger
    |
    |  Embedding generation
    v
MongoDB Atlas
    |
    v
Agentic RAG Explainer
    |
    |  Full-text + vector search
    |  Reciprocal Rank Fusion (RRF)
    v
Context-aware answer

The robot generates ROS 2 data that is streamed through Apache Kafka and stored as accountable data in MongoDB Atlas.

When a new document is inserted into MongoDB Atlas, an Atlas Trigger is automatically activated to generate its corresponding embedding. The embedding is stored together with the document and is subsequently used for vector search.

The Agentic RAG Explainer retrieves evidence from MongoDB Atlas using hybrid search, combining full-text and vector search. The Retriever applies Reciprocal Rank Fusion (RRF) to combine the results from both retrieval methods before passing the retrieved evidence to the subsequent stages of the Agentic RAG workflow. The workflow then uses the retrieved evidence to generate context-aware answers.
