use nu_engine::command_prelude::*;
use nu_protocol::ListStream;
use std::num::NonZeroUsize;

#[derive(Clone)]
pub struct Chain;

impl Command for Chain {
    fn name(&self) -> &str {
        "chain"
    }

    fn signature(&self) -> Signature {
        Signature::build("chain")
            .input_output_types(vec![
                // (Type::Nothing, Type::List(Box::new(Type::Any)),)
                (Type::Nothing, Type::ListStream,)
            ])
            .required("iterables", SyntaxShape::List(Box::new(SyntaxShape::Any)), "List of iterables that will be chained together.")
            .category(Category::Filters)
    }

    fn description(&self) -> &str {
        "Chain multiple iterables into one."
    }

    fn extra_description(&self) -> &str {
        ""
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["batch", "group"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "chain [1..5 10..15]",
                description: "Chaining two ranges into a single iterable",
                result: Some(Value::test_list(vec![
                    Value::test_list(vec![Value::test_int(1), Value::test_int(2)]),
                    Value::test_list(vec![Value::test_int(3), Value::test_int(4)]),
                ])),
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let chunk_size: Value = call.req(engine_state, stack, 0)?;

        let size =
            usize::try_from(chunk_size.as_int()?).map_err(|_| ShellError::NeedsPositiveValue {
                span: chunk_size.span(),
            })?;

        let size = NonZeroUsize::try_from(size).map_err(|_| ShellError::IncorrectValue {
            msg: "`chunk_size` cannot be zero".into(),
            val_span: chunk_size.span(),
            call_span: head,
        })?;

        chain(engine_state, input, size, head)
    }
}

pub fn chain(
    engine_state: &EngineState,
    input: PipelineData,
    chunk_size: NonZeroUsize,
    span: Span,
) -> Result<PipelineData, ShellError> {
    match input {
        PipelineData::Value(Value::List { vals, .. }, metadata) => {
            let chunks = ChainInter::new(vals, chunk_size, span);
            let stream = ListStream::new(chunks, span, engine_state.signals().clone());
            Ok(PipelineData::ListStream(stream, metadata))
        }
        PipelineData::ListStream(stream, metadata) => {
            let stream = stream.modify(|iter| ChainInter::new(iter, chunk_size, span));
            Ok(PipelineData::ListStream(stream, metadata))
        }
        input => Err(input.unsupported_input_error("list", span)),
    }
}

struct ChainInter<I: Iterator<Item = Value>> {
    iter: I,
    size: usize,
    span: Span,
}

impl<I: Iterator<Item = Value>> ChainInter<I> {
    fn new(iter: impl IntoIterator<IntoIter = I>, size: NonZeroUsize, span: Span) -> Self {
        Self {
            iter: iter.into_iter(),
            size: size.into(),
            span,
        }
    }
}

impl<I: Iterator<Item = Value>> Iterator for ChainInter<I> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let mut chunk = Vec::with_capacity(self.size); // delay allocation to optimize for empty iter
        chunk.push(first);
        chunk.extend((&mut self.iter).take(self.size - 1));
        Some(Value::list(chunk, self.span))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Chain {})
    }
}