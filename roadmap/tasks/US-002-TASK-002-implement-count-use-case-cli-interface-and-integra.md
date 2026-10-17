# Task: Implement count use case, CLI interface, and integration tests

- **ID**: `US-002-TASK-002`
- **Story ID**: `US-002`
- **Status**: `PENDING`
- **Change Type**: `FEATURE`
- **Depends On**: `US-002-TASK-001`
- **Target Files**: `src/application/count_usecase.rs`, `src/application/mod.rs`, `src/infrastructure/cli.rs`, `src/infrastructure/output.rs`, `src/main.rs`, `tests/integration_default.rs`

## Description

Implement count use case in src/application/count_usecase.rs. Support lines, words, and bytes options. Set default flags when no flags exist. Implement canonical output formatting in src/infrastructure/output.rs. Wire CLI arguments in src/infrastructure/cli.rs and src/main.rs. Keep main.rs as a thin shell. Add integration tests in tests/integration_default.rs to verify file and stdin outputs.
