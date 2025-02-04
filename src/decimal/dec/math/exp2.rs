use crate::decimal::{
    dec::{
        convert::to_i32,
        math::{exp::exp, mul::mul, powi::powi},
    },
    Decimal, Signal,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn exp2<const N: usize>(n: D<N>) -> D<N> {
    if n.is_nan() {
        return n.raise_signal(Signal::OP_INVALID);
    }

    if n.is_zero() {
        return D::ONE.with_ctx(n.context());
    }

    if n.is_integral() {
        if let Ok(n) = to_i32(n) {
            return powi(D::TWO, n);
        }
    }

    exp(mul(n, D::LN_2))
}
