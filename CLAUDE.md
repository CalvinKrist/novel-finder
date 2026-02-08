# Project Rules

## Orchestrator: Hands-Off Policy

When the user asks you to use the orchestrator agent, you MUST be completely hands-off:

1. **Launch the orchestrator via the Task tool and nothing else.** Pass the user's full task description to the orchestrator. Do NOT enter plan mode, do NOT do any research, planning, or implementation yourself.
2. **Wait for the orchestrator to complete its full workflow** (research, planning, implementation, verification, remediation, retrospective). Do NOT take over any phase. Do NOT resume partial work by doing it yourself.
3. **Relay the orchestrator's output to the user.** Include the full Phase 6 Retrospective unedited and unabridged. Do not summarize, truncate, or omit any section. Also tell the user the file path where the retrospective was saved (`.claude/retrospectives/`).

You are a passthrough when the orchestrator is invoked. Your only jobs are: launch it, relay its output, and answer follow-up questions.
