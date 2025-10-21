#!/usr/bin/env bash

# Check if gh is installed.
if ! command -v gh &> /dev/null; then
  echo "Error: gh (GitHub CLI) is not installed" >&2
  exit 1
fi

# Check if we're in a git repo.
if ! git rev-parse --git-dir &> /dev/null; then
  echo "Error: not in a git repository" >&2
  exit 1
fi

# Fetch current labels.
LABELS=$(gh api repos/:owner/:repo/labels)

# Update label if it exists and needs changes.
update_label() {
  local old_name="$1"
  local new_name="$2"
  local desc="$3"
  local color="$4"

  # Check if old label exists.
  if echo "$LABELS" | jq -e --arg name "$old_name" '.[] | select(.name == $name)' > /dev/null; then
    gh api --method PATCH "repos/:owner/:repo/labels/$old_name" \
      -f new_name="$new_name" \
      -f description="$desc" \
      -f color="$color"
  fi
}

# Create label if it doesn't already exist.
create_label() {
  local name="$1"
  local desc="$2"
  local color="$3"

  if ! echo "$LABELS" | jq -e --arg name "$name" '.[] | select(.name == $name)' > /dev/null; then
    gh label create "$name" --description "$desc" --color "$color"
  fi
}

# Update GitHub's default labels with emojis.
update_label "bug" "🐛 bug" "Something isn't working" "d73a4a"
update_label "enhancement" "✨ enhancement" "New feature or request" "a2eeef"
update_label "documentation" "📚 documentation" "Improvements or additions to documentation" "0075ca"
update_label "duplicate" "🔗 duplicate" "This issue or pull request already exists" "cfd3d7"
update_label "invalid" "⚠️ invalid" "This doesn't seem right" "e4e669"
update_label "question" "❓ question" "Further information is requested" "d876e3"
update_label "good first issue" "🎯 good first issue" "Good for newcomers" "7057ff"
update_label "help wanted" "🙋 help wanted" "Extra attention is needed" "008672"
update_label "wontfix" "⛔ wontfix" "This will not be worked on" "ffffff"

# Create additional labels.
create_label "♻️ refactor" "This is a refactor" "0e8a16"
create_label "🔒 security" "Security vulnerabilities or fixes" "191cdf"
create_label "🔧 tooling" "Build tools, CI/CD, dev workflow" "3edf05"
create_label "📌 task" "Actionable task or TODO item" "f97316"
