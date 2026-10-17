# Task: Implement StreamCounter and reader factory with unit tests

- **ID**: `US-002-TASK-001`
- **Story ID**: `US-002`
- **Status**: `PENDING`
- **Change Type**: `FEATURE`
- **Depends On**: `US-001-TASK-001`
- **Target Files**: `src/domain/count_strategy.rs`, `src/infrastructure/readers.rs`

## Description

Implement single-pass StreamCounter in src/domain/count_strategy.rs. Support lines, words, and bytes. Count Unicode whitespace with char::is_whitespace. Stream data in fixed-size 64KB buffers. Implement reader factory in src/infrastructure/readers.rs returning Box<dyn Read> for files and stdin. Add unit tests for empty input, newlines, and words.
