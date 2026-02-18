Continue work in this repository with focus on validation timeouts and subagent orchestration.

Primary target file:
- cloud_prompts.md

Objectives:
1) Continue/complete validation workflow while reducing timeout failures.
2) Update cloud_prompts.md with concrete operational guidance and commands for timeout-safe execution.
3) Use subagents carefully and cap parallel subagents at 3 total at any time.
4) Be memory-safe: avoid huge context loads and avoid spawning extra workers beyond the cap.
5) Prefer incremental batches/checkpoints so long runs can resume safely.

Constraints:
- NON-INTERACTIVE execution.
- Do not exceed 3 concurrent subagents.
- If a longer task is needed, split into phases and persist progress artifacts.

Deliverables:
- Updated cloud_prompts.md with improved timeout/subagent strategy.
- Any supporting small script/config changes needed for reliable execution.
- Brief summary of what changed and why.
