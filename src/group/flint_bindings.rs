/* automatically generated by rust-bindgen */

pub type mp_limb_signed_t = ::std::os::raw::c_long;
pub type mp_limb_t = ::std::os::raw::c_ulong;

impl From<&Mpz> for __mpz_struct {
    fn from(x: &Mpz) -> Self {
        __mpz_struct {
            _mp_alloc: x.inner.alloc,
            _mp_size: x.inner.size,
            _mp_d: x.inner.d,
        }
    }
}

#[test]
fn bindgen_test_layout___mpz_struct() {
    assert_eq!(
        ::std::mem::size_of::<__mpz_struct>(),
        16usize,
        concat!("Size of: ", stringify!(__mpz_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<__mpz_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(__mpz_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mpz_struct>()))._mp_alloc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mpz_struct),
            "::",
            stringify!(_mp_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mpz_struct>()))._mp_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__mpz_struct),
            "::",
            stringify!(_mp_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mpz_struct>()))._mp_d as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__mpz_struct),
            "::",
            stringify!(_mp_d)
        )
    );
}
pub type mpz_t = [__mpz_struct; 1usize];
pub type fmpz = mp_limb_signed_t;
pub type fmpz_t = [fmpz; 1usize];
extern "C" {
    #[link_name = "\u{1}_fmpz_get_mpz"]
    pub fn fmpz_get_mpz(x: *mut __mpz_struct, f: *mut fmpz);
}
extern "C" {
    #[link_name = "\u{1}_fmpz_set_mpz"]
    pub fn fmpz_set_mpz(f: *mut fmpz, x: *mut __mpz_struct);
}
extern "C" {
    #[link_name = "\u{1}_fmpz_xgcd_partial"]
    pub fn fmpz_xgcd_partial(
        co2: *mut fmpz,
        co1: *mut fmpz,
        r2: *mut fmpz,
        r1: *mut fmpz,
        L: *mut fmpz,
    );
}