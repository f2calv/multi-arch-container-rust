# Copilot Instructions

## Shared Instructions

Shared Copilot instruction files are maintained centrally in the [.github](https://github.com/f2calv/.github) repository under `instructions/`, and are applied to every workspace from the VS Code user profile via `~/.copilot/instructions`. They are deliberately not copied into this repository, so a change there takes effect everywhere without a pull request here.

Everything below is specific to this repository.

## Sibling Repositories (alignment is a hard requirement)

Four repositories implement the *same* trivial worker application in four languages:

- [multi-arch-container-dotnet](https://github.com/f2calv/multi-arch-container-dotnet)
- [multi-arch-container-go](https://github.com/f2calv/multi-arch-container-go)
- [multi-arch-container-rust](https://github.com/f2calv/multi-arch-container-rust) (this one)
- [multi-arch-container-python](https://github.com/f2calv/multi-arch-container-python)

Their premise is that a developer fluent in one language can learn another language's containerisation story by diffing two repositories. **Any change made here must be considered for the other two.** Keep the following as close to identical as possible:

- Repository layout and file names.
- `Dockerfile` stage names (`build`, `final`), section comment banners and ordering.
- The `ARG`/`ENV` provenance block and OCI `LABEL` block.
- Environment variable names consumed by the application — both the flat `GIT_*`/`GITHUB_*` provenance variables and the `APP__*` configuration overrides.
- Application file responsibilities: configuration model, logging setup, worker loop, entry-point wiring.
- `.github/workflows/ci.yml` job names and structure.
- `.editorconfig` common section, `.pre-commit-config.yaml`, `.devcontainer/`, `.vscode/extensions.json`.
- `build.sh` / `build.ps1` are byte-identical (all values are derived from git).
- `README.md` section headings.

## No Helm Charts

These repositories are **application code only**. Kubernetes packaging lives in the standalone [f2calv/helm-charts](https://github.com/f2calv/helm-charts) repository, which provides a single multi-purpose chart used by all deployments. Do not reintroduce a `charts/` directory or a `chart` job in `ci.yml`.

## Configuration Key Casing

Configuration keys are **snake_case** in both `appsettings.json` and the environment. This is deliberate: the `config` crate lower-cases environment keys but preserves file keys verbatim, so snake_case is the only casing where both sources resolve to the same key — and it is what the sibling .NET and Go repositories use. Do not "correct" them to camelCase or PascalCase. The keys themselves are documented in the [README](../README.md#configuration).

## Container Conventions

- The `TARGETARCH`+`TARGETVARIANT` to target-triple mapping is resolved **once** in the toolchain layer and written to `/etc/rust-target.env`, which later layers source. Do not duplicate the `case` statement.
- The final image is `gcr.io/distroless/cc-debian12:nonroot`. `cc` (not `static`) is required because the `*-unknown-linux-gnu` targets link dynamically against glibc.
- The Dockerfile builds with `--locked`, so `Cargo.lock` must be regenerated (`cargo fetch`) whenever `Cargo.toml` changes.
