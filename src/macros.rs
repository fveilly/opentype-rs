/// Helper macro for unwrapping `Option` values while returning early with an
/// error if the value of the expression is `None`. Can only be used in
/// functions that return `Option` because of the early return of `None` that
/// it provides.
#[macro_export]
macro_rules! try_opt {
    ($e:expr) =>(
        match $e {
            Some(v) => v,
            None => return None,
        }
    )
}