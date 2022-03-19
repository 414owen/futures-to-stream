//! Macros to create streams from heterogeneous futures

use futures::future::Future;
use futures::stream::Stream;

pub mod internal {
    use futures::future::Either;

    /// Create an `Either<A, A>::Left` from an `A`
    pub fn left_homogenous<A>(a: A) -> Either<A, A> {
        Either::Left(a)
    }
}

/// Create an iterator of homogeneous [`Future`]s from a set of heterogeneous futures
/// with the same associated [`Output`](Future::Output) type.
#[macro_export]
macro_rules! create_homogeneous_future {
  () => {
    {
      use std::option::*;
      use futures::future::Either;
      Either::Right(empty())
    }
  };
  ($x:expr) => {
    {
      Some($crate::internal::left_homogenous($x)).into_iter()
    }
  };
  ($x:expr, $($tail:expr),*) => {
    {
      use futures::future::Either;
      use std::iter::Iterator;

      Some(Either::Left($x)).into_iter().chain(
        $crate::create_homogeneous_future!($($tail),*).map(Either::Right)
      )
    }
  };
}

/// Create a [`Stream`] from a set of [`Future`]s, where all yielded [`Item`](Stream::Item)s are in the
/// order of their presented [`Future`]s.
#[macro_export]
macro_rules! futures_to_ordered_stream {
  ($($tail:tt)*) => {
    {
      use futures::stream::FuturesOrdered;

      let futs = $crate::create_homogeneous_future!($($tail)*);
      FuturesOrdered::from_iter(futs)
    }
  }
}

/// Create a [`Stream`] from a set of [`Future`]s, where [`Item`](Stream::Item)s may be yielded in any order.
#[macro_export]
macro_rules! futures_to_unordered_stream {
  ($($tail:tt)*) => {
    {
      use futures::stream::FuturesUnordered;

      let futs = $crate::create_homogeneous_future!($($tail)*);
      FuturesUnordered::from_iter(futs)
    }
  }
}

#[cfg(test)]
mod tests {
    use futures::stream::{Stream, StreamExt};
    use tokio::time::{sleep, Duration};

    async fn test1() -> u8 {
        sleep(Duration::from_millis(400)).await;
        1
    }
    async fn test2() -> u8 {
        sleep(Duration::from_millis(300)).await;
        2
    }
    async fn test3() -> u8 {
        sleep(Duration::from_millis(200)).await;
        3
    }
    async fn test4() -> u8 {
        sleep(Duration::from_millis(100)).await;
        4
    }

    fn futs_ordered() -> impl Stream<Item = u8> {
        futures_to_ordered_stream!(test1(), test2(), test3(), test4())
    }

    fn futs_unordered() -> impl Stream<Item = u8> {
        futures_to_unordered_stream!(test1(), test2(), test3(), test4())
    }

    #[tokio::test]
    async fn test_ordered() {
        let res: Vec<u8> = futs_ordered().collect().await;
        assert_eq!(vec![1, 2, 3, 4], res);
    }

    #[tokio::test]
    async fn test_unordered() {
        let res: Vec<u8> = futs_unordered().collect().await;
        assert_eq!(vec![4, 3, 2, 1], res);
    }
}
