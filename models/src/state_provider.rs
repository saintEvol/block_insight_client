use dioxus::prelude::{use_context, use_context_provider};

pub trait StateProvider {
    // fn use_context_provider() -> Self;
    fn use_context() -> Self;
}

impl<T> StateProvider for T
where
    T: Clone + 'static,
{
    // fn use_context_provider() -> T {
    //     let t = T::default();
    //     use_context_provider(|| t)
    // }

    fn use_context() -> T {
        use_context()
    }
}
