---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*), Bash(git diff:*)
description: Stage and commit using this repo's commit message convention
---

# Context

Current branch:
!`git branch --show-current`

Git status:
!`git status`

Staged + unstaged diff:
!`git diff HEAD`

Recent commits:
!`git log --oneline -5`

# Task

1. Extract the ticket number from the current branch name using pattern: leading digits before the first `-` (e.g. `289-sim-custom-topic-injection-mode` -> `#289`)
2. Stage all relevant changes (prefer naming specific files over `git add -A`)
3. Write a commit message in the format: `#NUMBER - description` — example: `#225 - added simulation black/whitelist based on topic name, using regex`
4. Commit in a single operation using a HEREDOC for the message
