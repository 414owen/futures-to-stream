macro_rules! _futs {
  () => {
    {
      use std::option::*;
      use futures::future::Either;
      Some(Either::Right(empty())).into_iter()
    }
  };
  ($x:expr) => {
    {
      use futures::future::Either;
      fn left<A>(a: A) -> Either<A, A> {
        Either::Left(a)
      }
      Some(left($x)).into_iter()
    }
  };
  ( $x:expr, $($tail:expr),* ) => {
    {
      use futures::future::Either;
      use std::iter::Iterator;

      Some(Either::Left($x)).into_iter().chain(
        _futs!($($tail),*).map(Either::Right)
      )
    }
  };
}

#[macro_export]
macro_rules! futures_to_stream {
  ( $($tail:tt)* ) => {
    {
      use futures::stream::FuturesOrdered;

      let fs = _futs!($($tail)*);
      FuturesOrdered::from_iter(fs)
    }
  }
}

#[cfg(test)]
mod tests {
  use futures::stream::{Stream, StreamExt};

  async fn test1() -> () { () }
  async fn test2() -> () { () }
  async fn test3() -> () { () }
  async fn test4() -> () { () }
  
  fn futs() -> impl Stream<Item = ()> {
    futures_to_stream!(
      test1(),
      test2(),
      test3(),
      test4()
    )
  }

  #[tokio::test]
  async fn test() {
    let res: Vec<()> = futs().collect().await;
    assert_eq!(vec![(), (), (), ()], res);
  }
}
