use dioxus::prelude::{use_context, use_context_provider};
use utils::context_provider::ContextProvider;

/// 可以全局使用的服务
/// 默认是使用[use_context_provider]进入注入，因此instance中应该只使用各种hook如[use_signal]进行各数据的初始化
pub trait Service: Clone + 'static {
    /// 初始化实例并不是在任何hook中进行的，因此如果要获取其它hook，需要使用use_类接口进行获取，请勿手动构建，如使用:Signal::new()等
    /// 初始化实例应该只进行收集所需要的相关hook数据，而不要进行其它计算
    fn instance() -> Self;
}

pub trait ServiceProvider {
    fn init() -> Self;
    fn use_service() -> Self;
}

impl<T> ServiceProvider for T
where
    T: Service,
{
    /// 初始化服务，如果需要在整个应用都可用，请确保在应用最开始处调用
    fn init() -> T {
        let t = <Self as Service>::instance();
        use_context_provider(|| t)
    }

    /// 使用服务
    fn use_service() -> T {
        use_context()
    }
}
