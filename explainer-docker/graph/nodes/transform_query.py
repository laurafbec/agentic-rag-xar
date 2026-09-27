from typing import Any, Dict

from graph.chains.question_rewriter import question_rewriter_chain
from graph.state import GraphState


def transform_query(state: GraphState) -> Dict[str, Any]:
    print("---TRANSFORM QUERY---")
    question = state["question"]
    documents = state["documents"]

    history = state.get("history", [])

    better_question = question_rewriter_chain.invoke({"question": question})

    loop_step = state.get("loop_step", 0) + 1
    print(f"Updating loop step {state.get('loop_step', 0)} -> {loop_step}")

    return {"documents": documents, "question": better_question, "history": history, "loop_step": loop_step}