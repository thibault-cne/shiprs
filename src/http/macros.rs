//! Utility macros from the `httparse` crate.
//! https://github.com/seanmonstar/httparse/blob/master/src/macros.rs

macro_rules! next {
    ($bytes:ident => $ret:expr) => {{
        match $bytes.next() {
            Some(b) => b,
            None => return $ret,
        }
    }};
}

macro_rules! expect {
    ($bytes:ident.next() == $pat:pat => $ret:expr) => {
        expect!(next!($bytes => $ret) => $pat => $ret)
    };
    ($e:expr => $pat:pat => $ret:expr) => {
        match $e {
            v@$pat => v,
            _ => return $ret
        }
    };
}

macro_rules! space {
    ($bytes:ident or $err:expr) => ({
        expect!($bytes.next() == b' ' => Err($err));
        $bytes.slice();
    })
}

macro_rules! newline {
    ($bytes:ident) => ({
        match next!($bytes => Err(NewLine.into())) {
            b'\r' => {
                expect!($bytes.next() == b'\n' => Err(NewLine.into()));
                $bytes.slice();
            },
            b'\n' => {
                $bytes.slice();
            },
            _ => return Err(NewLine.into())
        }
    })
}
