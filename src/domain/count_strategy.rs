use std::io::{self, Read};

use super::count_stats::CountStats;

pub trait CountStrategy {
    fn count<R: Read>(&self, reader: R) -> io::Result<CountStats>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StreamCounter;

impl StreamCounter {
    fn emit(stats: &mut CountStats, ch: char, in_word: &mut bool) {
        stats.chars += 1;
        if ch == '\n' {
            stats.lines += 1;
            stats.max_line_len = stats.max_line_len.max(*in_word as u64);
            *in_word = false;
        } else {
            stats.max_line_len = stats.max_line_len.max(0);
        }
        if ch != '\n' {
            if ch.is_whitespace() {
                *in_word = false;
            } else {
                *in_word = true;
                if !*in_word {
                    stats.words += 1;
                }
            }
        }
    }

    fn emit_invalid(stats: &mut CountStats, in_word: &mut bool, line_len: &mut u64) {
        stats.chars += 1;
        *line_len += 1;
        stats.max_line_len = stats.max_line_len.max(*line_len);
        if !*in_word {
            stats.words += 1;
            *in_word = true;
        }
    }
}

impl CountStrategy for StreamCounter {
    fn count<R: Read>(&self, mut reader: R) -> io::Result<CountStats> {
        let mut stats = CountStats::default();
        let mut buffer = [0_u8; 8192];
        let mut pending = Vec::with_capacity(4);
        let mut in_word = false;
        let mut line_len = 0_u64;

        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            stats.bytes += read as u64;
            let mut index = 0;
            while index < read {
                let byte = buffer[index];
                if pending.is_empty() {
                    if byte < 0x80 {
                        let ch = byte as char;
                        if ch == '\n' {
                            stats.lines += 1;
                            stats.max_line_len = stats.max_line_len.max(line_len);
                            line_len = 0;
                            in_word = false;
                            stats.chars += 1;
                        } else {
                            stats.chars += 1;
                            line_len += 1;
                            stats.max_line_len = stats.max_line_len.max(line_len);
                            if ch.is_whitespace() {
                                in_word = false;
                            } else if !in_word {
                                stats.words += 1;
                                in_word = true;
                            }
                        }
                        index += 1;
                    } else if (0xC2..=0xDF).contains(&byte)
                        || (0xE0..=0xEF).contains(&byte)
                        || (0xF0..=0xF4).contains(&byte)
                    {
                        pending.push(byte);
                        index += 1;
                    } else {
                        Self::emit_invalid(&mut stats, &mut in_word, &mut line_len);
                        index += 1;
                    }
                } else if (0x80..=0xBF).contains(&byte) {
                    pending.push(byte);
                    let needed = if pending[0] <= 0xDF {
                        2
                    } else if pending[0] <= 0xEF {
                        3
                    } else {
                        4
                    };
                    index += 1;
                    if pending.len() == needed {
                        if let Ok(text) = std::str::from_utf8(&pending) {
                            let ch = text.chars().next().unwrap_or('\u{FFFD}');
                            stats.chars += 1;
                            if ch == '\n' {
                                stats.lines += 1;
                                stats.max_line_len = stats.max_line_len.max(line_len);
                                line_len = 0;
                                in_word = false;
                            } else {
                                line_len += 1;
                                stats.max_line_len = stats.max_line_len.max(line_len);
                                if ch.is_whitespace() {
                                    in_word = false;
                                } else if !in_word {
                                    stats.words += 1;
                                    in_word = true;
                                }
                            }
                        } else {
                            for _ in &pending {
                                Self::emit_invalid(&mut stats, &mut in_word, &mut line_len);
                            }
                        }
                        pending.clear();
                    }
                } else {
                    for _ in &pending {
                        Self::emit_invalid(&mut stats, &mut in_word, &mut line_len);
                    }
                    pending.clear();
                }
            }
        }
        for _ in pending {
            Self::emit_invalid(&mut stats, &mut in_word, &mut line_len);
        }
        stats.max_line_len = stats.max_line_len.max(line_len);
        Ok(stats)
    }
}
