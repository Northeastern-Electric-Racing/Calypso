---
allowed-tools: Bash(git:*), Bash(gh pr view:*), Bash(gh pr edit:*), Bash(cargo fmt:*), Bash(cargo clippy:*), Bash(cargo test:*), Bash(cargo build:*)
description: Update the current branch's PR title, body, and checks after new commits
---

## Context

Current branch:
!`git branch --show-current`

Recent commits since Develop:
!`git log --oneline Develop..HEAD`

## Task

Run each step in order. Stop and report if any step fails.

### 1. Get the current PR

```bash
gh pr view --json number,title,body,commits,reviews
```

If no PR exists for this branch, stop and tell the user to run `/open-pr` first.

### 2. Run lint and tests

```bash
cargo fmt --all --check
cargo clippy --verbose --all -- -D warnings
cargo test --verbose
```

If any check fails, stop and report the failure. Do not push broken code.

### 3. Push latest changes

```bash
git push
```

### 4. Update PR title and body

Extract the ticket number from the branch name using the leading digits before the first `-` (e.g. `289-sim-custom-topic-injection-mode` -> `#289`).

Read the full diff with `git diff Develop..HEAD` and the full commit log with `git log Develop..HEAD --oneline`.

Use `.github/pull_request_template.md` as the base for the updated body. Follow the same style rules as `/open-pr`:

- **Changes**: 1-2 concise sentences. What was added/changed and why, not a line-by-line summary. Reflect ALL commits on the branch, not just the latest.
- **Notes**: Brief, informal context — design decisions, tradeoffs, things the reviewer should know. Remove this section if nothing to say.
- **Test Cases**: Practical CLI examples showing how to verify. Include expected behavior for each.
- **To Do**: Remove this section unless there are actual remaining items.
- **Checklist**: Check off all applicable items. Always check "Remove any non-applicable sections."
- **Tone**: Casual and direct. No formal language or over-explanation.
- End with `Closes #TICKET` (ticket number from branch prefix).

Write the filled-in body to `/tmp/pr-body.md`.

Update the PR:
```bash
gh pr edit --title "#TICKET - brief title" --body-file /tmp/pr-body.md
```

Keep the title under 70 characters. The title should summarize the full scope of the branch, not just the latest commit.

### 5. Report

Output the updated PR URL:
```bash
gh pr view --json url --jq '.url'
```
