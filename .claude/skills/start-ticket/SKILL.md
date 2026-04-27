---
name: start-ticket
description: Read a ticket, create a branch, and implement the work end-to-end
user-invocable: true
allowed-tools: Bash, Read, Edit, Write, Grep, Glob
---

# Start Ticket

Pick up a GitHub Issue, create a branch, and implement the work fully.

## Steps

### 1. Select the ticket

If a ticket number was provided as an argument, use that. Otherwise, list recent open issues and ask the user which one to work on (this is the ONLY time to ask):

```bash
gh issue list --limit 15 --state open
```

### 2. Read the ticket and self-assign

```bash
gh issue view <NUMBER>
gh issue edit <NUMBER> --add-assignee @me
```

Carefully read the full issue body, acceptance criteria, and any linked discussions.

### 3. Create the branch

Derive the branch name using the pattern `{number}-{kebab-case-title}` (e.g., ticket 533 titled "Fix login redirect" becomes `533-fix-login-redirect`).

Create the branch from the latest `Develop`:

```bash
git checkout Develop && git pull origin Develop && git checkout -b <branch>
```

### 4. Implement the ticket

Use `/lap` repeatedly — each lap finds the highest-leverage improvement, makes it, verifies it, and commits. Keep running laps until every requirement in the ticket is fully met.

### 5. Done

Output a one-line summary of what was built.
