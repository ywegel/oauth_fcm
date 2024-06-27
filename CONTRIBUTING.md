## How to contribute

If you have questions, ask them in the Discussion tab on github. If you have found a bug or have a feature request, create an issue
with the correct label.

## Making a change

1. If your change will be visible to crate consumers, create a GitHub issue and link it to your pull request.
2. Fork and clone the repository.
3. Create a new branch: `git checkout -b my-branch-name`, with a name that summarises your change.
4. Make your change, format your changes with `cargo +nightly fmt` and add tests if possible.
5. Add an entry to the *unreleased* section of CHANGELOG.md using this pattern if your change is visible to crate
   consumers: ```- Short summary of your change (#GitHub issue number)```.
   No changelog entry and GitHub issue is needed, if it is only an internal change or documentation
6. Push to your fork, submit a pull request and make sure that the CI passes.
7. Wait for your pull request to be reviewed and merged.

## Licensing

By contributing to this project, you agree that your contributions will be licensed under the same license as the
project (see LICENSE file). All new code must be submitted under this license.