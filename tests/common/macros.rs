macro_rules! success_ref {
    ($res:expr) => {{
        assert!($res.is_success());
        $res.success_ref().unwrap()
    }};
}
