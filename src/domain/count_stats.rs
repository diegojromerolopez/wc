#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CountStats {
    pub lines: u64,
    pub words: u64,
    pub bytes: u64,
    pub chars: u64,
    pub max_line_len: u64,
}

#[cfg(test)]
mod tests {
    use super::CountStats;

    #[test]
    fn default_stats_are_zero_and_nonzero_stats_differ() {
        let default_stats = CountStats::default();
        assert_eq!(
            default_stats,
            CountStats {
                lines: 0,
                words: 0,
                bytes: 0,
                chars: 0,
                max_line_len: 0,
            }
        );
        assert_ne!(
            default_stats,
            CountStats {
                lines: 1,
                ..CountStats::default()
            }
        );
    }
}
