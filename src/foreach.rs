
macro_rules! async_for {
    ($item:ident in $stream:ident $todo:block) => (
        while let Some($item) = await!($stream.next()) $todo
    )
}
