use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(usize)]
pub(crate) enum Turn {
    W,
    U,
}