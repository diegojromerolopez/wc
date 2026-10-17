# Task: Implement use case, CLI parsing, output formatting, and integration tests

- **ID**: `US-002-TASK-002`
- **Story ID**: `US-002`
- **Status**: `PENDING`
- **Change Type**: `FEATURE`
- **Depends On**: `US-002-TASK-001`
- **Target Files**: `src/application/count_usecase.rs`, `src/application/mod.rs`, `src/infrastructure/cli.rs`, `src/infrastructure/output.rs`, `src/infrastructure/mod.rs`, `src/main.rs`, `tests/integration_default.rs`

## Description

Implement CLI flags -l, -w, -c with clap in src/infrastructure/cli.rs. Default to lines, words, and bytes. Implement CountUseCase in src/application/count_usecase.rs. Format counts in canonical order using {:>7} in src/infrastructure/output.rs. Wire thin main function in src/main.rs. Add integration tests in tests/integration_default.rs using assert_cmd.
