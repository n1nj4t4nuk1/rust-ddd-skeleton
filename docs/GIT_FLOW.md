# Git Flow

This project follows a simplified **git-flow** branching strategy.

The branching strategy is intentionally simple: `main` always contains production-ready code, `dev` is the integration branch where features are merged and tested together, and feature branches are short-lived branches where individual changes are developed. This keeps the git history clean and makes it easy to understand what changed and when.

All development happens on feature branches created from `dev`. When a feature is complete, it is merged back into `dev` with `--no-ff` to preserve the branch history. When `dev` is stable and ready for release, it is merged into `main` and tagged.

---

## Branches

| Branch | Purpose |
|---|---|
| `main` | Production-ready code. Only receives merges from `dev`. |
| `dev` | Integration branch. All features merge here first. |
| `feature/<name>` | One branch per feature, created from `dev`. |
| `hotfix/<name>` | Critical production fixes, branched from `main`. |

---

## Feature workflow

```
dev
 └── feature/<name>   <-- all commits for this feature go here
      │
      └── merge back into dev when done
```

### Step by step

```bash
# 1. Start a new feature from dev
git checkout dev
git checkout -b feature/<name>

# 2. Work and commit
git add ...
git commit -m "feat(...): ..."

# 3. Merge into dev when done (no fast-forward to keep history readable)
git checkout dev
git merge --no-ff feature/<name>
git branch -d feature/<name>
```

---

## Hotfix workflow

Hotfixes are an exception to the normal flow. They are used only for urgent production issues that cannot wait for the next release cycle. Hotfixes branch from `main` (not `dev`) and are merged back into both `main` and `dev` to ensure the fix is present in both branches.

```bash
# 1. Branch from main
git checkout main
git checkout -b hotfix/<name>

# 2. Fix and commit
git commit -m "fix(...): ..."

# 3. Merge into both main and dev
git checkout main
git merge --no-ff hotfix/<name>

git checkout dev
git merge --no-ff hotfix/<name>

git branch -d hotfix/<name>
```

---

## Promoting dev to main

When `dev` is stable and ready for release:

```bash
git checkout main
git merge --no-ff dev
git tag -a v<version> -m "Release v<version>"
```

---

## Commit message conventions

Consistent commit messages make the git history useful as documentation. The project follows Conventional Commits, which prefixes each message with a type (feat, fix, test, etc.) and an optional scope. This makes it easy to scan the history and understand what each commit does at a glance.

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

Types: feat, fix, test, refactor, docs, ci, chore
```

Examples:
```
feat(config): add update_config_entry use case
fix(config): handle empty value in ConfigValue constructor
test(config): add unit tests for config entry deleter
refactor(cqrs): simplify command bus registration
docs: add git-flow guide
ci: trigger workflows on main and dev branches
```
