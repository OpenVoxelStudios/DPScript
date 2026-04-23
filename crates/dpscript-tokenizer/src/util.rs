use peekmore::PeekMoreIterator;

pub trait PeekSized<I: Iterator> {
    fn peek_many<const N: usize>(&mut self) -> Option<[I::Item; N]>;
}

impl<I: Iterator> PeekSized<I> for PeekMoreIterator<I>
where
    I::Item: Copy,
{
    fn peek_many<const N: usize>(&mut self) -> Option<[<I as Iterator>::Item; N]> {
        self.peek_amount(N)
            .as_array::<N>()
            .unwrap()
            .try_map(|it| it)
    }
}
