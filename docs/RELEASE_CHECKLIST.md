# Release Checklist

## Purpose

Ensure consistency and quality before publishing a release.

## Checklist

- [ ] Update [CHANGELOG](CHANGELOG.md)
- [ ] Run `cargo test`
- [ ] Validate main examples in `examples/`
- [ ] Validate CLI (`lex`, `parse`, `check`)
- [ ] Review SPEC for inconsistencies
- [ ] Tag the version in Git (`vX.Y.Z`)
- [ ] Publish the release on GitHub
