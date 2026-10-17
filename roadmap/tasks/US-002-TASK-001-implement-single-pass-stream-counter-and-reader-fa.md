# Task: Implement single-pass stream counter and reader factory

- **ID**: `US-002-TASK-001`
- **Story ID**: `US-002`
- **Status**: `PENDING`
- **Change Type**: `FEATURE`
- **Depends On**: `US-001-TASK-001`
- **Target Files**: `src/domain/count_strategy.rs`, `src/infrastructure/readers.rs`, `src/infrastructure/mod.rs`

## Description

Implement StreamCounter in src/domain/count_strategy.rs. Count lines, words, and bytes in a single pass. Use buffered reading with 64KB chunks. Handle Unicode whitespace characters correctly. Implement readers.rs to open files or standard input as dynamic Read objects. Add unit tests with Cursor inputs to verify empty, newline, word, and whitespace sequences.
