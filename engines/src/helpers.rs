use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum Turn {
    W,
    U,
}