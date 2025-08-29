use dioxus::prelude::{use_context, use_context_provider};

/// 默认是使用[use_context_provider]进入注入，因此instance中应该只使用各种hook如[use_signal]进行各数据的初始化
pub trait ContextProvider: Clone + 'static {
    fn instance() -> Self;
}

pub trait AutoContextProvider {
    fn use_context_provider() -> Self;
    fn use_context() -> Self;
}

impl<T> AutoContextProvider for T
where
    T: ContextProvider,
{
    /// 注入
    fn use_context_provider() -> T {
        let t = <Self as ContextProvider>::instance();
        use_context_provider(|| t)
    }

    /// 使用
    fn use_context() -> T {
        use_context()
    }
}
