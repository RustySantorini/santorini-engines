use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(usize)]
enum Turn {
    W,
    U,
}