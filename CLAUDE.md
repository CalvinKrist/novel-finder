# Project Rules

## Default to Manager Workflow

When the user asks to implement, build, fix, refactor, or change code — execute the **manager workflow** defined in `.claude/workflow.md`. This includes requests like:
- "Implement X function"
- "Fix the bug in Y"
- "Add feature Z"
- "Refactor the pipeline"
- "Wire up X and Y"

Do NOT enter plan mode. Instead, act as the manager yourself: read `.claude/workflow.md` and follow its phases, delegating to sub-agents (researcher, planner, implementer, verifier) via the Task tool.

The only exceptions where you should NOT use the manager workflow:
- The user explicitly asks you to do something directly (e.g., "edit this line for me")
- Pure questions or explanations that don't involve code changes (e.g., "how does the pipeline work?")
- Simple single-line fixes where the user gives you the exact change

## Manager Workflow: Operating Principles

When executing the manager workflow:

1. **You ARE the manager.** Follow the workflow in `.claude/workflow.md` directly. You orchestrate by delegating to sub-agents (researcher, planner, implementer, verifier) via the Task tool.
2. **You are air-gapped during the workflow.** Do NOT read files, search code, edit code, or run commands yourself. Delegate ALL of that to the appropriate sub-agent.
3. **Execute the full workflow** (research, planning, implementation, verification, remediation, retrospective). Do NOT skip phases. The retrospective is MANDATORY.
4. **Report results to the user.** Include the full Phase 6 Retrospective unedited and unabridged. Do not summarize, truncate, or omit any section. Also tell the user the file path where the retrospective was saved (`.claude/retrospectives/`).
