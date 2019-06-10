use libc_enum::libc_enum;

mod libc {
    #[allow(non_camel_case_types)]
    pub type c_int = i32;

    pub const SIGINT: c_int = 2;
    pub const SIGTERM: c_int = 15;

    #[cfg(feature = "sigpwr")]
    pub const SIGPWR: c_int = 30;
}

#[libc_enum(libc::c_int)]
#[derive(Debug,PartialEq)]
pub enum Signal {
    SIGINT,
    SIGTERM,

    #[cfg(feature = "sigpwr")]
    SIGPWR,
}

#[test]
fn test_into_libc() {
    assert_eq!(libc::SIGINT, Signal::SIGINT.into());
    assert_eq!(libc::SIGTERM, Signal::SIGTERM.into());

    #[cfg(feature = "sigpwr")]
    assert_eq!(libc::SIGPWR, Signal::SIGPWR.into());
}

#[test]
fn test_try_from_libc() {
    use std::convert::TryFrom;

    assert_eq!(Ok(Signal::SIGINT), Signal::try_from(libc::SIGINT));
    assert_eq!(Ok(Signal::SIGTERM), Signal::try_from(libc::SIGTERM));

    #[cfg(feature = "sigpwr")]
    assert_eq!(Ok(Signal::SIGPWR), Signal::try_from(libc::SIGPWR));

    assert_eq!(Err(()), Signal::try_from(0));
}
