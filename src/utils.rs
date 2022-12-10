pub fn exec_ext() -> &'static str {
    if cfg!(windows) {
        ".cmd"
    } else {
        ""
    }
}
