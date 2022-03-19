# Futures to stream

![example workflow](https://github.com/414owen/futures-to-stream/actions/workflows/ci.yml/badge.svg)
![crates.io](https://img.shields.io/crates/v/futures-to-stream.svg)

Macros to create streams from heterogeneous futures

## Usage

```
async fn test1() -> u8 { 1 }
async fn test2() -> u8 { 2 }
async fn test3() -> u8 { 3 }
async fn test4() -> u8 { 4 }

fn ordered_stream() -> impl Stream<Item = u8> {
  futures_to_ordered_stream!(test1(), test2(), test3(), test4())
}

fn unordered_stream() -> impl Stream<Item = u8> {
  futures_to_unordered_stream!(test1(), test2(), test3(), test4())
}
```

## Goal

To allow the creation of a stream from heterogeneous
[`Future`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)s with equal
associated `Item`s.

## Problem

The way to create a
[`Stream`](https://docs.rs/futures/0.3.21/futures/stream/trait.Stream.html)
from a set of [`Future`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)s is to create a
[`FuturesOrdered`](https://docs.rs/futures/0.3.21/futures/stream/struct.FuturesOrdered.html), or a
[`FuturesUnordered`](https://docs.rs/futures/0.3.21/futures/stream/struct.FuturesUnordered.html).
However, as these store [`Future`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)s of
the type they're parameterized over, you can't give them a heterogeneous set of [`Futures`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)s.

Here's an example that compiles:

```
async fn test1() -> () { () }

fn to_stream() -> impl Stream<Item = ()> {
  let mut futs = FuturesOrdered::new();
  futs.push(test1());
  futs.push(test1());
  futs
}
```

Here's an example that doesn't compile:

```
async fn test1() -> () { () }

async fn test2() -> () { () }

fn to_stream() -> impl Stream<Item = ()> {
  let mut futs = FuturesOrdered::new();
  futs.push(test1());
  futs.push(test2()); // Error: expected opaque type, found a different opaque type
  futs
}
```

Great, very helpful rustc. We've created the exact same function under a different name,
and it's [`Future`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)
is different somehow.

Well, there is a way to combine two different futures pretty easily -- Use
[`future::Either`](https://docs.rs/futures/0.3.21/futures/future/enum.Either.html)

```
async fn test1() -> () { () }

async fn test2() -> () { () }

fn to_stream() -> impl Stream<Item = ()> {
  let mut futs = FuturesOrdered::new();
  futs.push(Either::Left(test1()));
  futs.push(Either::Right(test2()));
  futs
}
```

That's great, now let's try with four [`Future`](https://docs.rs/futures/0.3.21/futures/future/trait.Future.html)s.

```
async fn test1() -> () { () }
async fn test2() -> () { () }
async fn test3() -> () { () }
async fn test4() -> () { () }

fn to_stream() -> impl Stream<Item = ()> {
  let mut futs = FuturesOrdered::new();
  futs.push(Either::Left(test1()));
  futs.push(Either::Right(Either::Left(test2())));
  futs.push(Either::Right(Either::Right(Either::Left(test3()))));
  futs.push(Either::Right(Either::Right(Either::Right(test4()))));
  futs
}
```

With four, it's already pretty unwieldy. Luckily, this package exports macros to
generate this all for you.

[Back to `Usage`](#Usage)
