use futures::future::Either;

pub fn left_homogenous<A>(a: A) -> Either<A, A> {
  Either::Left(a)
}

#[macro_export]
macro_rules! futures_to_stream {
  (impl) => {
    {
      use std::option::*;
      use futures::future::Either;
      Either::Right(empty())
    }
  };
  (impl $x:expr) => {
    {
      Some($crate::left_homogenous($x)).into_iter()
    }
  };
  (impl $x:expr, $($tail:expr),*) => {
    {
      use futures::future::Either;
      use std::iter::Iterator;

      Some(Either::Left($x)).into_iter().chain(
        futures_to_stream!(impl $($tail),*).map(Either::Right)
      )
    }
  };
  ($($tail:tt)*) => {
    {
      use futures::stream::FuturesOrdered;

      let fs = futures_to_stream!(impl $($tail)*);
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
