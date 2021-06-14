//! Some of these objects don't expose enough to accurately report their debug state. In this case
//! we show as much state as we can. Users can always use `Debug2Format` to get more information,
//! at the cost of bringing core::fmt into the firmware and doing the layout work on device.
//!
//! We generally keep the type parameter trait bounds in case it becomes possible to use this
//! later, without making a backwards-incompatible change.

mod num;
mod ops;
mod slice;

use super::*;

impl<T> Format for Option<T>
where
    T: Format,
{
    default_format!();

    #[inline]
    fn _format_tag() -> u16 {
        internp!("None|Some({=?})")
    }

    #[inline]
    fn _format_data(&self, fmt: Formatter) {
        match self {
            None => fmt.inner.u8(&0),
            Some(x) => {
                fmt.inner.u8(&1);
                fmt.inner.tag(T::_format_tag());
                x._format_data(fmt)
            }
        }
    }
}

impl<T, E> Format for Result<T, E>
where
    T: Format,
    E: Format,
{
    default_format!();

    #[inline]
    fn _format_tag() -> u16 {
        internp!("Err({=?})|Ok({=?})")
    }

    #[inline]
    fn _format_data(&self, fmt: Formatter) {
        match self {
            Err(e) => {
                fmt.inner.u8(&0);
                fmt.inner.tag(E::_format_tag());
                e._format_data(fmt)
            }
            Ok(x) => {
                fmt.inner.u8(&1);
                fmt.inner.tag(T::_format_tag());
                x._format_data(fmt)
            }
        }
    }
}

impl<T> Format for core::marker::PhantomData<T> {
    default_format!();

    #[inline]
    fn _format_tag() -> u16 {
        internp!("PhantomData")
    }

    #[inline]
    fn _format_data(&self, _fmt: Formatter) {}
}

impl Format for core::convert::Infallible {
    default_format!();

    #[inline]
    fn _format_tag() -> u16 {
        unreachable!();
    }

    #[inline]
    fn _format_data(&self, _fmt: Formatter) {
        unreachable!();
    }
}

impl Format for core::time::Duration {
    fn format(&self, fmt: Formatter) {
        crate::write!(
            fmt,
            "Duration {{ secs: {=u64}, nanos: {=u32} }}",
            self.as_secs(),
            self.subsec_nanos(),
        )
    }
}

impl<A, B> Format for core::iter::Zip<A, B>
where
    A: Format,
    B: Format,
{
    fn format(&self, fmt: Formatter) {
        crate::write!(fmt, "Zip(..)")
    }
}
