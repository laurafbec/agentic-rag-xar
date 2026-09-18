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

-v "$(pwd)/questions_answers.json:/app/questions_answers.json:ro"

The questions correspond to those used in the evaluation of the associated study, allowing the same queries to be processed again in an unattended execution.



