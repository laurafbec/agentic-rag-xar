# Accountability-Explainability Architecture with LLM Agents
This repository contains the code for an accountability and explainability architecture developed for ROS 2-based robots. It leverages Kafka as an event streaming platform to ensure the reliable recording of accountable data, which serves as the foundation for an Agentic RAG-based system that generates natural language explanations.

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
