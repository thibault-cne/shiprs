macro_rules! success_ref {
    ($res:expr) => {{
        assert!($res.is_success());
        $res.clone().success().unwrap()
    }};
}
