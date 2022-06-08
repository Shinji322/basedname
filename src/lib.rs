pub fn truncate(arg: String, matches: u8) -> String {
    match arg.rmatch_indices('.').nth(matches as usize - 1) {
        Some(index) => {
            arg[..index.0].to_string()
        },
        None => {
            match arg.match_indices('.').next() {
                Some(index) => {
                    arg[..index.0].to_string()
                },
                None => {
                    arg.to_string()
                }
            }
        }
    }
}
