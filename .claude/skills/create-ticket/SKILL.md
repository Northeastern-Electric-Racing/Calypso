---
name: create-ticket
description: Create a new GitHub Issue from a description
user-invocable: true
allowed-tools: Bash, Read, Grep, Glob
---

# Create Ticket

Create a new GitHub Issue with proper formatting and labels.

## Steps

### 1. Get the description

If the user provided a description as an argument, use that. Otherwise, ask what the ticket should cover (this is the ONLY time to ask).

### 2. Classify and pick a template

Based on the description, classify the work as one of the repo's issue types:
- **bug** — something broken
- **feature-request** — new capability
- **task** — general work item
- **epic** — large multi-issue effort
- **spike** — research / investigation
- **other** — anything else

### 3. Generate the issue

- **Title**: Use the pattern `[Area] - Short Description` (e.g. `[SIM] - Add custom topic injection mode`)
- **Body**: Include a clear description, acceptance criteria, and any relevant context. Follow the structure from the matching `.github/ISSUE_TEMPLATE/` template.
- **Labels**: Apply the most relevant label(s) from the repo's actual label list (run `gh label list` if unsure). Common ones: `bug`, `good first issue`, `epic`, `dependencies`, `submodules`, `rust`, `github_actions`. Don't invent labels — pick from what exists.

### 3a. Write like a teammate, not like an AI

Use plain English. Short sentences. The reader is another engineer skimming a backlog, not a marketing audience.

**Avoid these tells:**
- "compose" / "compose freely" / "can't compose" → "run together", "run at the same time", "work alongside each other"
- "control plane" / "primitive" / "unlock" / "first-class" / "surface area" / "blast radius" → describe the thing in plain words
- "agent-driven way to drive X" / "X-driven Y" stacked → pick one phrasing
- "yields cleanly" / "honors the registry" / "mediates ownership" → "skips", "checks", "tracks"
- "elegant" / "clean" / "robust" / "powerful" / "seamless" → describe what makes it good instead, or drop the adjective
- Empty connective tissue: "the key insight is", "the unlock here is", "fundamentally"
- Stacking tables, nested headings, and bullet sub-lists when a short paragraph would do

**Rewrite test:** if a sentence has a phrase you wouldn't say out loud to a coworker, rewrite it.

**Length:** short sections. Three short paragraphs beat one long one. Acceptance criteria as a checklist beats prose.

### 4. Create the issue

```bash
gh issue create --title "<title>" --label "<label>" --body "$(cat <<'EOF'
<body>
EOF
)"
```

### 5. Done

Output the issue number and URL.
