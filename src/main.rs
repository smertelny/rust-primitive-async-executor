use std::task::{ Context, RawWakerVTable, RawWaker, Waker, Poll };
use std::future::Future;
use std::pin::Pin;

static VTABLE: &RawWakerVTable = &RawWakerVTable::new(
    |data: *const ()| RawWaker::new(data, VTABLE),
    |_data: *const ()| {},
    |_data: *const ()| {},
    |_data: *const ()| {},
);

async fn first() {
    println!("First");
}

async fn second() {
    first().await;
    println!("Second");
}


fn main() {
    let waker = unsafe {
        Waker::from_raw(RawWaker::new(&{}.into(), VTABLE))
    };
    let mut ctx = Context::from_waker(&waker);
    let mut pinned_fn:Pin<Box<dyn Future<Output=()>>> = Box::pin(second());
    if let Poll::Ready(result) = pinned_fn.as_mut().poll(&mut ctx) {
        dbg!(result);
    };
}