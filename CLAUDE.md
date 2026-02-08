# Project Rules

## Default to Manager Agent

When the user asks to implement, build, fix, refactor, or change code — delegate the ENTIRE task to the **manager** agent (`subagent_type: "manager"`). This includes requests like:
- "Implement X function"
- "Fix the bug in Y"
- "Add feature Z"
- "Refactor the pipeline"
- "Wire up X and Y"

Do NOT enter plan mode, do NOT research the codebase yourself, do NOT write any code yourself. Just launch the manager agent with the user's full request and let it handle everything end-to-end.

The only exceptions where you should NOT use the manager agent:
- The user explicitly asks you to do something directly (e.g., "edit this line for me")
- Pure questions or explanations that don't involve code changes (e.g., "how does the pipeline work?")
- Simple single-line fixes where the user gives you the exact change

## Manager Agent: Hands-Off Policy

When the manager agent is running, you MUST be completely hands-off:

1. **Launch the manager via the Task tool and nothing else.** Pass the user's full task description. Do NOT enter plan mode, do NOT do any research, planning, or implementation yourself.
2. **Wait for the manager to complete its full workflow** (research, planning, implementation, verification, remediation, retrospective). Do NOT take over any phase. Do NOT resume partial work by doing it yourself.
3. **Relay the manager's output to the user.** Include the full Phase 6 Retrospective unedited and unabridged. Do not summarize, truncate, or omit any section. Also tell the user the file path where the retrospective was saved (`.claude/retrospectives/`).

You are a passthrough when the manager is invoked. Your only jobs are: launch it, relay its output, and answer follow-up questions.
