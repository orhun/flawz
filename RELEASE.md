# Creating a Release

The releases are automated via [`release-plz`](https://github.com/MarcoIeni/release-plz), [`cargo-dist`](https://github.com/axodotdev/cargo-dist), [`git-cliff`](https://github.com/orhun/git-cliff) and [GitHub Actions](https://docs.github.com/en/actions). See [this blog post](https://blog.orhun.dev/automated-rust-releases/) for more information.

1. `release-plz` creates a pull request for the new releases automatically. So simply merge the [release PR](https://release-plz.ieni.dev/docs/usage/release-pr) for updating the crate versions and changelog.

2. After the merge, `release-plz` will pick up the new version and automatically create a tag. This will trigger `cargo-dist` and release binaries will be built in via [release workflow](.github/workflows/release.yml).

3. Mark the package out-of-date for package managers.

4. Announce the release on social platforms. 🥳
