---
allowed-tools: Bash(git:*), Bash(gh pr create:*), Bash(cargo fmt:*), Bash(cargo clippy:*), Bash(cargo test:*), Bash(cargo build:*)
description: Run pre-PR checks, push the branch, and open a pull request
---

## Your task

Run each step in order. Stop and report if any step fails.

### 1. Verify commit format

!`git log --oneline Develop..HEAD`

Confirm all commits match `#TICKET - description` (ticket number extracted from branch prefix digits before first `-`). Flag any that don't.

### 2. Lint and test
```bash
cargo fmt --all --check
cargo clippy --verbose --all -- -D warnings
cargo test --verbose
```

### 3. Conflict check
```bash
git fetch origin Develop
git merge --no-commit --no-ff origin/Develop
git merge --abort
```
If conflicts exist, stop and ask the user to resolve.

### 4. Write PR body

Use `.github/pull_request_template.md` as the base. Read the full diff with `git diff Develop..HEAD`.

**PR style for this repo:**
- **Changes**: 1-2 concise sentences. What was added/changed and why, not a line-by-line summary.
- **Notes**: Brief, informal context — design decisions, tradeoffs, things the reviewer should know. Remove this section if nothing to say.
- **Test Cases**: Practical CLI examples showing how to verify (e.g. `simulate --topic "BMS/Pack/Voltage=30000"`). Include expected behavior for each.
- **To Do**: Remove this section unless there are actual remaining items.
- **Checklist**: Check off all applicable items. Always check "Remove any non-applicable sections."
- **Tone**: Casual and direct. No formal language or over-explanation.
- End with `Closes #TICKET` (ticket number from branch prefix).

Write the filled-in body to `/tmp/pr-body.md`.

### 5. Push and open PR
```bash
git push -u origin $(git branch --show-current)
gh pr create --draft --base Develop --title "#TICKET - brief title" --body-file /tmp/pr-body.md
```

Use the ticket number extracted from the branch name. Keep the title under 70 characters. Report the PR URL when done.
