use rational_impl_types::Rational;
use std::{mem, ptr::NonNull, str::FromStr};

#[export_name = "rational_impl_is_dynamically_linked"]
pub unsafe extern "C" fn is_dynamically_linked() -> i32 {
    if cfg!(feature = "link_static") {
        0
    } else {
        1
    }
}

#[export_name = "rational_impl_get_version"]
pub unsafe extern "C" fn get_version(major: *mut i32, minor: *mut i32, patch: *mut i32) {
    let maj = i32::from_str(env!("CARGO_PKG_VERSION_MAJOR")).unwrap_or_default();
    let min = i32::from_str(env!("CARGO_PKG_VERSION_MINOR")).unwrap_or_default();
    let pat = i32::from_str(env!("CARGO_PKG_VERSION_PATCH")).unwrap_or_default();

    if let Some(mut major) = NonNull::new(major) {
        *(major.as_mut()) = maj;
    }

    if let Some(mut minor) = NonNull::new(minor) {
        *(minor.as_mut()) = min;
    }

    if let Some(mut patch) = NonNull::new(patch) {
        *(patch.as_mut()) = pat;
    }
}

#[export_name = "rational_impl_add_rationals"]
pub unsafe extern "C" fn add_rationals(rat1: *const Rational, rat2: *const Rational) -> Rational {
    let mut rat1 = unsafe { unwrap_rational_ptr(rat1) };
    let mut rat2 = unsafe { unwrap_rational_ptr(rat2) };

    if rat1.denominator == 0 || rat2.denominator == 0 {
        // We have a logical divide by zero - this is an error.
        return Default::default();
    }

    normalize_two(&mut rat1, &mut rat2);

    let mut ret = Rational {
        numerator: rat1.numerator + rat2.numerator,
        denominator: rat1.denominator,
    };

    normalize_one(&mut ret);

    ret
}

#[export_name = "rational_impl_normalize_rational"]
pub unsafe extern "C" fn normalize_rational(rat: *mut Rational) {
    if let Some(mut rational) = NonNull::new(rat) {
        normalize_one(rational.as_mut());
    }
}

#[inline]
unsafe fn unwrap_rational_ptr(rational: *const Rational) -> Rational {
    NonNull::new(rational as *mut Rational)
        .map(|p| unsafe { *NonNull::as_ref(&p) })
        .unwrap_or_default()
}

fn normalize_two(x: &mut Rational, y: &mut Rational) {
    let (dx, dy) = (x.denominator, y.denominator);
    if dx == dy {
        return;
    }

    let lcm = lcm(dx, dy);
    let norm = |r: &mut Rational| {
        let denom = r.denominator;
        if lcm != denom {
            let factor = lcm / denom;
            r.numerator *= factor;
            r.denominator = lcm;
        }
    };

    norm(x);
    norm(y);
}

fn normalize_one(x: &mut Rational) {
    let gcd = gcd(x.numerator, x.denominator);
    x.numerator /= gcd;
    x.denominator /= gcd;
}

fn lcm(mut x: u64, mut y: u64) -> u64 {
    if y > x {
        mem::swap(&mut x, &mut y);
    }

    if y == 0 {
        0
    } else {
        x * (y / gcd(x, y))
    }
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    if y > x {
        mem::swap(&mut x, &mut y);
    }

    while y != 0 {
        x %= y;
        mem::swap(&mut x, &mut y);
    }

    x
}
