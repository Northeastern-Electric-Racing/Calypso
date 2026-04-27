---
name: review
description: "Hierarchical code review: triage, review, score, fix, simplify. Run /review to review branch changes, /review improve to learn from this session."
argument-hint: "[improve]"
allowed-tools: Read, Write, Edit, Glob, Grep, Bash(git *), Bash(gh *), Agent
---

# Code Review

Hierarchical review pipeline modeled after Anthropic's code-review plugin. Cheap triage, focused review, independent scoring, then fix and simplify.

## Argument Handling

If `` is "improve", skip to **Step 7: Improve** below.

---

## Step 1: Eligibility Check

Use a **Haiku agent** (`model: "haiku"`). Determine if there is anything to review:

- Get the base branch from CLAUDE.md conventions (default: `develop` if found, otherwise `main`)
- Run `git diff --name-only $(git merge-base HEAD <base-branch>)...HEAD`
- If no changed files, tell the user and stop

## Step 2: Gather Context

Use a **Haiku agent** (`model: "haiku"`). Return:

- List of all changed file paths, categorized (source, test, config, docs, types)
- Paths to relevant CLAUDE.md files (root + any in directories containing changed files)
- Contents of `.claude/review-rules.md` if it exists

## Step 3: Summarize the Change

Use a **Haiku agent** (`model: "haiku"`):

- Run `git diff $(git merge-base HEAD <base-branch>)...HEAD`
- Return a brief summary of what this change does and why (intent, not implementation)

Steps 1-3 can run in parallel.

---

## Step 4: Review

Read every changed file in full (not just the diff) for surrounding context.

Launch **parallel agents** to independently review the change. Each agent returns a list of issues with the reason each was flagged (CLAUDE.md adherence, bug, historical context, etc.).

### Agent dispatch:

| # | Agent | Condition | What it does |
|---|---|---|---|
| A | `code-reviewer` (PR Review Toolkit) | Always | CLAUDE.md compliance, bug detection, general code quality. Scores 0-100, reports 80+. |
| B | Shallow bug scan | Always | Read just the diff. Scan for obvious bugs -- logic errors, null safety, async issues, wrong operators. Focus on big bugs, skip nitpicks. Ignore likely false positives. |
| C | Git history context | Always | Read `git blame` and history of modified code. Identify bugs in light of historical context. |
| D | `silent-failure-hunter` (PR Review Toolkit) | Error handling code present (catch blocks, .catch(), Result types, fallback logic) | Audit error handlers for silent failures, logging quality, catch specificity. |
| E | `comment-analyzer` (PR Review Toolkit) | Comments or docs added/modified | Verify comment accuracy against code, flag misleading docs. |

Agents B and C are inline (not toolkit) -- use `model: "sonnet"` for these.

### False positive examples (pass to agents B and C verbatim):

Do NOT flag:
- Pre-existing issues (not introduced by this change)
- Pedantic nitpicks a senior engineer wouldn't call out
- Functionality changes clearly intentional given the broader change
- Real issues on lines the user did not modify

---

## Step 5: Confidence Scoring

For **each issue** found in Step 4, launch a parallel **Haiku agent** (`model: "haiku"`) that takes the issue, the PR diff, and the CLAUDE.md files, and returns a confidence score.

For issues flagged due to CLAUDE.md, the agent must double-check that the CLAUDE.md actually calls out that issue specifically.

### Scoring rubric (pass to each agent verbatim):

- **0**: Not confident at all. False positive that doesn't stand up to light scrutiny, or is a pre-existing issue.
- **25**: Somewhat confident. Might be real, might be false positive. Agent wasn't able to verify. If stylistic, not explicitly called out in CLAUDE.md.
- **50**: Moderately confident. Verified real issue, but may be a nitpick or rare in practice. Not very important relative to the rest of the PR.
- **75**: Highly confident. Double-checked and verified it is very likely real and will be hit in practice. The existing approach in the PR is insufficient. Directly impacts functionality, or directly mentioned in CLAUDE.md.
- **100**: Absolutely certain. Double-checked and confirmed definitely real, will happen frequently. Evidence directly confirms this.

### After scoring:

**Filter out all issues with confidence < 80.** If nothing survives, report no high-confidence issues found and skip to Step 6b (Simplify).

---

## Step 6: Fix & Simplify

### 6a. Auto-Fix

Present surviving issues grouped by severity:

- **Critical** (90-100): Will cause incorrect behavior, data loss, or crash
- **Important** (80-89): Significant issue requiring attention

For each issue:
- Description and confidence score
- File path and line number
- Why it's an issue (cite specific CLAUDE.md rule or provide bug evidence)

**Fix all Critical and Important issues directly.** Briefly note what was changed for each.

If any issues scored 50-79, list under **Risks** -- state without fixing.

### 6b. Simplify

After fixes are applied, dispatch the `code-simplifier` agent (PR Review Toolkit) with the current state of all changed files. It simplifies for clarity, consistency, and maintainability while preserving functionality. Apply its suggestions directly.

### 6c. Summary

- **Strengths**: What the change does well
- **Fixed**: List of issues with scores and what was changed
- **Risks**: Issues below threshold worth knowing about
- **Verdict**: Is this ready to merge?

---

## Step 7: Improve

**This runs when the user invokes `/review improve`.**

This phase makes the skill itself better by editing this SKILL.md file directly. Read the full SKILL.md first, then review the conversation history since the last `/review` was run.

### Analyze the session:

1. **False positives**: What did `/review` flag that the user ignored, dismissed, or reverted? Should the false-positive list or scoring rubric be adjusted?
2. **Missed issues**: What did the user fix on their own that `/review` didn't catch? Should an agent condition be added or the scoring rubric adjusted?
3. **Overreach**: Did `/review` rate something too high? Should thresholds change?
4. **Good calls**: What findings did the user agree with and keep?

### Apply changes to this SKILL.md:

- **Edit** agent dispatch conditions, scoring rubric, or false-positive guidance that produced bad results
- **Remove** rules that are consistently wrong and can't be salvaged
- **Add** new false-positive examples or dispatch conditions when something was missed
- **Adjust** scoring thresholds if findings were consistently over- or under-rated

### Constraints:

- Do not change the overall structure (steps, agent dispatch pattern, scoring pipeline)
- Do not remove the Step 7: Improve section itself
- Do not change the frontmatter
- Keep edits surgical -- every change must trace to a specific session outcome

After editing, summarize what was changed and why.
