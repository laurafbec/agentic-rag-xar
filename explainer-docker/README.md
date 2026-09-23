# Agentic RAG-based Explainer
Docker-based implementation of the Agentic RAG explainer for generating context-aware explanations of autonomous robot behavior.

The explainer processes accountable robot data stored in MongoDB Atlas Vector and uses an Agentic RAG workflow to retrieve relevant evidence and generate explanations based on the available context.

## Requirements
- Access to the MongoDB Atlas database containing the accountable robot data.
- API credentials for the configured language model services.

## Configuration
Create a ```.env``` file in this directory with the credentials and configuration required by the explainer.
The configuration includes the connection parameters for MongoDB Atlas and the API credentials required by the language model services.

## Input questions
The default execution mode processes a predefined set of questions without requiring manual interaction.

The questions are provided in [questions_answers.json](./questions_answers.json).

Each entry contains two files. For example:

```
{
  "user_input": "How many goals were successfully achieved by the robot?",
  "reference": "The robot successfully achieved 3 goals."
}
```
The ```user_input``` field contains the question submitted to the Agentic RAG workflow. The ```reference``` field contains the corresponding reference answer used for subsequent evaluation.

The reference answers are not provided to the Agentic RAG workflow when generating the explanations. They are retained to support the evaluation of the generated responses.

The file is mounted into the container as read-only:

```-v "$(pwd)/questions_answers.json:/app/questions_answers.json:ro"```

The questions correspond to those used in the evaluation of the associated study, allowing the same queries to be processed again in an unattended execution.

## Agentic RAG workflow
The explainer implements an Agentic RAG workflow in which the retrieved evidence is evaluated and the query can be reformulated when the retrieved information is insufficient.

The workflow includes the following components:

1. Retriever
Retrieves relevant evidence using hybrid search (RRF) from the accountable robot data stored in MongoDB Atlas.

2. Retrieval Grader
Evaluates whether the retrieved evidence is relevant to the question.

3. Query Rewriter
Reformulates the query when the retrieved evidence is not considered sufficient.

4. Answer Generator
Generates an answer based on the retrieved evidence.

5. Hallucination Grader
Evaluates whether the generated answer is supported by the retrieved evidence.

6. Answer Grader
Checks whether the generated answer addresses the user question.

If the retrieved documents are not relevant, the query is transformed and the retrieval process is repeated. If the generated answer is not grounded in the retrieved evidence, it is regenerated. If the answer does not adequately address the question, the query is transformed and the retrieval process is repeated.

The workflow is implemented using LangGraph and maintains the conversation state during execution.



## Run with Docker
### Build the Docker image
From the ```explainer-docker``` directory, build the Docker image:

```
docker build -t explainer-docker .
```

### Run the explainer
Run the explainer using the configuration in ```.env```:

```
docker run --rm \
 --env-file .env \
 -v "$(pwd)/questions_answers.json:/app/questions_answers.json:ro" \
 -v "$(pwd)/evaluation/datasets:/app/evaluation/datasets" \
 explainer-docker
```
The explainer reads each user_input from questions_answers.json, retrieves relevant evidence from MongoDB Atlas, and executes the Agentic RAG workflow to generate an answer.

The execution is unattended and processes the predefined questions automatically.

## Run locally
The explainer can also be executed directly in a Python environment without Docker.
### First deployment
Create and activate the Conda environment and install the required dependencies:
```
conda create -n rt_explainer python=3.12.0
conda activate rt_explainer
pip install -r requirements.txt
```
Configure the required environment variables in ```.env``` and ensure that the MongoDB Atlas database containing the accountable robot data is accessible.

### Subsequent executions
Activate the environment and run the application:
```
conda activate rt_explainer
python3 main.py
```
## Example output
A typical execution reports the main stages of the workflow:
```
When did the robot complete the navigation task?
---RETRIEVE---
'Finished running: retrieve:'
---CHECK DOCUMENT RELEVANCE TO QUESTION---
---GRADE: DOCUMENT RELEVANT---
---GRADE: DOCUMENT NOT RELEVANT---
---GRADE: DOCUMENT RELEVANT---
---GRADE: DOCUMENT NOT RELEVANT---
---GRADE: DOCUMENT RELEVANT---
---ASSESS GRADED DOCUMENTS---
---- Loop step-----: 1
---DECISION: GENERATE---
'Finished running: grade_documents:'
---GENERATE---
---CHECK HALLUCINATIONS---
---- Conversation History-----:
Why did the robot replan the route?
The robot replanned the route because there was an obstacle blocking the original planned path when attempting to reach **goal pose number 2**. To successfully achieve this goal, the trajectory was dynamically adjusted to avoid the obstacle. No other reasons for replanning are mentioned in the provided context.
Which topic publish the position and orientation of the robot?
The topic that publishes the position and orientation of the robot is **`amcl_pose`**. This is explicitly stated in the `topic_name` field of all the provided context documents, which include entries for position (x, y) and orientation (z, w) values.
What was the linear velocity of the robot when navigating to goal pose number 2?
The linear velocity of the robot when navigating to **goal pose number 2** was **0.26 m/s in the x-direction** (with y-component 0.0). This is derived from the `cmd_vel` topic data in the context, specifically the document with `rank: 2`, which corresponds to the velocity command during navigation to this goal. No other linear velocities are explicitly tied to goal 2 in the provided context.
---DECISION: GENERATION IS GROUNDED IN DOCUMENTS---
---GRADE GENERATION vs QUESTION---
---DECISION: GENERATION ADDRESSES QUESTION---
'Finished running: generate:'
Documents content: ['1739117071.817763065 Navigation to the goal number 1 has been aborted.', '1739117156.635998838 Navigation to the goal number 2 has succeeded.', '1739117199.73031389 Navigation to the goal number 3 has succeeded.']
('The robot ended navigation tasks at the following timestamps:  \n'
 '- **Goal 1 (aborted)**: Ended at **2025-02-09 16:04:31**.  \n'
 '- **Goal 2 (succeeded)**: Ended at **2025-02-09 16:05:56**.  \n'
 '- **Goal 3 (succeeded)**: Ended at **2025-02-09 16:06:39**.  \n'
 '\n'
 'These timestamps correspond to the completion (success or abortion) of each '
 'navigation goal as recorded in the provided context. The final navigation '
 'task concluded with goal 3 succeeding at the latest timestamp.')
```

## Execution and evaluation workflow
The relationship between execution and evaluation can be summarized as follows:
```text

questions_answers.json
        |
        v
   user_input
        |
        v
Agentic RAG Explainer
        |
        +--------------------+
        |                    |
        v                    v
 retrieved_contexts       response
        |                    |
        +---------+----------+
                  |
                  v
              reference
                  |
                  v
          RAGAS evaluation
```

## Evaluation data
The evaluation process uses the generated responses together with the reference answers and the retrieved evidence.

Evaluation records contain the following fields:

```
 {
        "user_input": "How many goals were successfully achieved by the robot?",
        "reference": "The robot successfully achieved 3 goals.",
        "retrieved_contexts": [
            "1739813746.634414401 Navigation to the goal number 1 has succeeded.",
            "1739813830.235360667 Navigation to the goal number 2 has succeeded.",
            "1739813867.155290475 Navigation to the goal number 3 has succeeded."
        ],
        "response": "The robot successfully achieved 3 goals. The context provided indicates that navigation to goal numbers 1, 2, and 3 each succeeded."
    }
```
The fields are used as follows:
- ```user_input```: question submitted to the Agentic RAG workflow.
- ```reference```: expected answer used as a reference for evaluation.-
- ```retrieved_contexts```: evidence retrieved from the accountable robot data.
- ```response```: answer generated by the Agentic RAG workflow.
- 
The resulting evaluation data can be used to assess retrieval and answer quality with RAGAS. RAGAS is part of the evaluation process and is not required for the normal execution of the explainer.

The evaluation datasets are located in [evaluation/datasets/](./evaluation/datasets/)

This directory can be mounted into the container when required for evaluation-related processing.

## Reproducibility
The Docker image provides an isolated environment for executing the Agentic RAG explainer with the software dependencies required by the application.

To reproduce an execution:
1. Configure the required credentials in ```.env```.
2. Ensure that the MongoDB Atlas database containing the accountable robot data is accessible.
3. Provide the ```questions_answers.json``` file.
4. Build the Docker image.
5. Run the container using the command described above.

## Relation to the complete architecture
This component corresponds to the explainability stage of the accountability and explainability architecture described in the associated publication.

The complete architecture integrates:
- ROS 2 for robot operation and data generation.
- Apache Kafka for accountable data streaming.
- MongoDB Atlas for storing and retrieving accountable data.
- An Agentic RAG workflow for evidence retrieval and answer generation.

The Docker components required for the other stages of the architecture are provided in the corresponding directories of the main repository.

