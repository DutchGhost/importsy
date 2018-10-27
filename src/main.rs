#![feature(await_macro, async_await, futures_api, pin, existential_type)]

extern crate tokio;
extern crate tokio_stdin_stdout;
extern crate futures;

use tokio::prelude::{AsyncWriteExt, StreamExt, Stream};

use std::marker::Unpin;

#[macro_use]
mod foreach;


async fn stdout_daemon<S: Stream + Unpin>(
    mut receiver: S,
    func: impl Fn(&S::Item) -> &[u8],
) -> Result<(), S::Error> {
    let mut stdout = tokio_stdin_stdout::stdout(1024);

    async_for!(item in receiver {
        let unwrapped = item?;
        let bytes = func(&unwrapped);
        await!(stdout.write_async(bytes));
        await!(stdout.write_async(b"\n"));
    });

    await!(stdout.flush_async());

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
