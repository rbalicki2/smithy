use std::iter::repeat_with;

fn optionalize_and_extend_with_none<T>(
  iter: impl Iterator<Item = T>,
) -> impl Iterator<Item = Option<T>> {
  iter.map(|item| Some(item)).chain(repeat_with(|| None))
}

pub fn optionalize_and_zip<T, U>(
  left_iter: impl ExactSizeIterator<Item = T>,
  right_iter: impl ExactSizeIterator<Item = U>,
) -> impl Iterator<Item = (Option<T>, Option<U>)> {
  let max_len = std::cmp::max(left_iter.len(), right_iter.len());
  let left_optionalized = optionalize_and_extend_with_none(left_iter);
  let right_optionalized = optionalize_and_extend_with_none(right_iter);
  left_optionalized.zip(right_optionalized).take(max_len)
}

// TODO figure out why this doesn't compile
// pub fn optionalize_zip_reverse_and_enumerate<T, U> (
//   left_iter: impl ExactSizeIterator<Item = T>,
//   right_iter: impl ExactSizeIterator<Item = U>,
// ) -> impl Iterator<Item = (usize, (Option<T>, Option<U>))> {
//   let should_reverse = left_iter.len() > right_iter.len();
//   let zipped = optionalize_and_zip(left_iter, right_iter).enumerate();

//   let ret: Box<dyn Iterator<Item = (usize, (Option<T>, Option<U>))>> = if should_reverse {
//     let mut vec: Vec<(usize, (Option<T>, Option<U>))> = zipped.collect();
//     vec.reverse();
//     Box::new(vec.into_iter())
//   } else {
//     Box::new(zipped)
//   };

//   ret.map(|(i, (l, r))| (i, (l, r)))
// }
