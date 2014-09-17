
#![macro_escape]

///////////////////////////////////////////////////////////////////////
/// evaluate the expression, then check for GL error.
macro_rules! glcheck(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)
