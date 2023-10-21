pub mod iter;
pub mod mutation_diff {
    #[derive(tabled::Tabled)]
    pub struct MutationDiff {
        pub old: MaybeDiffEntry,
        pub new: MaybeDiffEntry,
        pub is_diff: MaybeDiffEntry,
        // log: MaybeDiffEntry,
    }

    impl<T> From<Option<T>> for MaybeDiffEntry
    where
        T: std::fmt::Debug,
    {
        fn from(value: Option<T>) -> Self {
            Self(value.map(|v| format!("{v:?}")))
        }
    }

    #[derive(Default)]
    pub struct MaybeDiffEntry(Option<String>);

    impl std::fmt::Display for MaybeDiffEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0.as_ref() {
                Some(v) => v.fmt(f),
                None => "~".fmt(f),
            }
        }
    }
}
