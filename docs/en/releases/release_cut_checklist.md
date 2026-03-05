# Release Cut Checklist

Use this checklist when cutting a new release tag.

## Before tag

- [ ] Release Draft is up-to-date and grouped by labels.
- [ ] CHANGELOG was updated from the draft.
- [ ] Required checks are green on `main`.
- [ ] Local validation passed: `./scripts/ci-local.sh`.
- [ ] No open blockers in high-priority issues.

## Tag and publish

- [ ] Tag created: `vX.Y.Z`.
- [ ] Tag pushed to origin.
- [ ] GitHub Release created from draft.

## After release

- [ ] Announce release in team channels.
- [ ] Open follow-up issues for deferred items.
- [ ] Confirm next milestone scope.
