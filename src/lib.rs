pub mod solutions {
    pub mod y2024 {
        use seq_macro::seq;

        seq!(DAY in 01..=31 {
            pub mod day~DAY;
        });
    }
}
