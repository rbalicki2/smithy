macro_rules! many_0_custom(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use ::nom::lib::std::result::Result::*;
      // use ::nom::{Err,AtEof};
      use ::nom::Err;

      // ret is Ok|Err
      let ret;
      let mut vec_of_responses = ::nom::lib::std::vec::Vec::new();
      let mut input = $i.clone();

      loop {
        let input_ = input.clone();
        match $submac!(input_, $($args)*) {
          Ok((i, o))              => {
            // i is remaining
            // o is matched

            // N.B. I don't know if this is actually solves the infinite loops...

            // loop trip must always consume (otherwise infinite loops)
            if i.len() == 0 || i.len() == input.len() {
              vec_of_responses.push(o);
              ret = Ok((input, vec_of_responses));
              break;
            }
            // if i == input {
            //   if i.at_eof() {
            //     ret = Ok((input, res));
            //   } else {
            //     ret = Err(Err::Error(error_position!(input, ::nom::ErrorKind::Many0)));
            //   }
            //   break;
            // }
            vec_of_responses.push(o);

            input = i;
          },
          Err(Err::Error(_))      => {
            ret = Ok((input, vec_of_responses));
            break;
          },
          Err(e) => {
            ret = Err(e);
            break;
          },
        }
      }

      ret
    }
  );
  ($i:expr, $f:expr) => (
    many_0_custom!($i, call!($f));
  );
);

/// `many1!(I -> IResult<I,O>) => I -> IResult<I, Vec<O>>`
/// Applies the parser 1 or more times and returns the list of results in a Vec
///
/// the embedded parser may return Incomplete
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # fn main() {
///  named!(multi<&[u8], Vec<&[u8]> >, many1!( tag!( "abcd" ) ) );
///
///  let a = b"abcdabcdefgh";
///  let b = b"azerty";
///
///  let res = vec![&b"abcd"[..], &b"abcd"[..]];
///  assert_eq!(multi(&a[..]),Ok((&b"efgh"[..], res)));
///  assert_eq!(multi(&b[..]), Err(Err::Error(error_position!(&b[..], ErrorKind::Many1))));
/// # }
/// ```
macro_rules! many_1_custom(
  // N.B. I added a nom:: here before $submac. What is going on?
  ($i:expr, nom::$submac:ident!( $($args:tt)* )) => (
    {
      use nom::lib::std::result::Result::*;
      use nom::Err;

      use nom::InputLength;
      let i_ = $i.clone();
      match $submac!(i_, $($args)*) {
        Err(Err::Error(_))      => Err(Err::Error(
          nom::error_position!(i_, nom::ErrorKind::Many1)
        )),
        Err(Err::Failure(_))      => Err(Err::Failure(
          nom::error_position!(i_, nom::ErrorKind::Many1)
        )),
        Err(i) => Err(i),
        Ok((i1,o1))   => {
          let mut res    = nom::lib::std::vec::Vec::with_capacity(4);
          res.push(o1);
          let mut input  = i1;
          let mut error = nom::lib::std::option::Option::None;
          loop {
            let input_ = input.clone();
            match $submac!(input_, $($args)*) {
              Err(Err::Error(_))                    => {
                break;
              },
              Err(e) => {
                error = nom::lib::std::option::Option::Some(e);
                break;
              },
              Ok((i, o)) => {
                if i.input_len() == input.input_len() {
                  break;
                }
                res.push(o);
                input = i;
              }
            }
          }

          match error {
            nom::lib::std::option::Option::Some(e) => Err(e),
            nom::lib::std::option::Option::None    => Ok((input, res))
          }
        }
      }
    }
  );
  ($i:expr, $f:expr) => (
    many_1_custom!($i, nom::call!($f));
  );
);
