use std::kinds::marker;

use MutView;
use traits::{Matrix, OptionMutSlice};

impl<'a, 'b, T> OptionMutSlice<'b, (uint, uint), MutView<'b, T>> for MutView<'a, T> {
    fn mut_slice(&'b mut self, start: (uint, uint), end: (uint, uint)) -> Option<MutView<'b, T>> {
        let (end_row, end_col) = end;
        let (nrows, ncols) = self.size();
        let (start_row, start_col) = start;

        if end_col < ncols && end_col > start_col + 1 &&
                end_row < nrows && end_row > start_row + 1 {
            let stride = self.stride;
            let ptr = unsafe { self.data.offset((start_row * stride + start_col) as int) };

            Some(MutView {
                _contravariant: marker::ContravariantLifetime::<'a>,
                _nocopy: marker::NoCopy,
                _nosend: marker::NoSend,
                data: ptr,
                size: (end_row - start_row, end_col - start_col),
                stride: stride,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use quickcheck::TestResult;

    use test;
    use traits::{OptionMutSlice, OptionIndex, OptionIndexMut};

    #[quickcheck]
    fn at(
        (size, (row, col)): ((uint, uint), (uint, uint)),
        (start, end): ((uint, uint), (uint, uint)),
        (sub_start, sub_end): ((uint, uint), (uint, uint)),
    ) -> TestResult {
        match test::mat(size).as_mut().and_then(|m| {
            m.mut_slice(start, end)
        }).as_mut().and_then(|v| {
            v.mut_slice(sub_start, sub_end)
        }).as_ref().and_then(|v| v.at(&(row, col))) {
            None => TestResult::discard(),
            Some(e) => {
                let (start_row, start_col) = start;
                let (sub_start_row, sub_start_col) = sub_start;
                let col_ = sub_start_col + start_col + col;
                let row_ = sub_start_row + start_row + row;

                TestResult::from_bool((row_, col_).eq(e))
            },
        }
    }

    #[quickcheck]
    fn at_mut(
        (size, (row, col)): ((uint, uint), (uint, uint)),
        (start, end): ((uint, uint), (uint, uint)),
        (sub_start, sub_end): ((uint, uint), (uint, uint)),
    ) -> TestResult {
        match test::mat(size).as_mut().and_then(|m| {
            m.mut_slice(start, end)
        }).as_mut().and_then(|v| {
            v.mut_slice(sub_start, sub_end)
        }).as_mut().and_then(|v| v.at_mut(&(row, col))) {
            None => TestResult::discard(),
            Some(e) => {
                let (start_row, start_col) = start;
                let (sub_start_row, sub_start_col) = sub_start;
                let col_ = sub_start_col + start_col + col;
                let row_ = sub_start_row + start_row + row;

                TestResult::from_bool((row_, col_).eq(e))
            },
        }
    }
}
