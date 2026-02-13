# Release Checklist

## Purpose

Ensure consistency and quality before publishing a release.

## Checklist

- [ ] Update [CHANGELOG](CHANGELOG.md)
- [ ] Update [SUMMARY](SUMMARY.md)
- [ ] Run `cargo test`
- [ ] Run `markdownlint "**/*.md"`
- [ ] Validate main examples in `examples/`
- [ ] Validate CLI (`lex`, `parse`, `check`)
- [ ] Review SPEC for inconsistencies
- [ ] Tag the version in Git (`vX.Y.Z`)
- [ ] Publish the release on GitHub
