use std::{future::Future, marker::PhantomData};

/// Handlers

/// A trait that represents a handler.
/// A handler is a function that takes in some arguments, and returns a future that resolves to a
/// future.
///
/// It is not the handlers responsibility to convert the request into the arguments that it
/// requires. This is the responsibility of the service that is calling the handler.
pub trait Handler<Args>: Clone + Send + Sized + 'static {
    type Output;
    type Future: Future<Output = Self::Output> + Send + 'static;
    // We make this future-like so that it can handle both async (Axum) and sync (Actix).
    fn call(&self, args: Args) -> Self::Future;
}

/// A handle that wraps a handler, and can be used to call the handler.
/// This is useful for abstracting over different handler types.
/// We use PhantomData to carry over the types from our functions to whatever handler we are using.
#[derive(Clone, Copy)]
pub struct Handle<F, Args>(pub F, PhantomData<Args>)
where
    F: Handler<Args>;

impl<F, Args> Handle<F, Args>
where
    F: Handler<Args> + Clone + Send + Sync + 'static,
    Args: Clone + Send + Sync + 'static,
    F::Future: Send,
{
    /// Create a new handle.
    pub fn new(handler: F) -> Self {
        Self(handler, PhantomData)
    }
}

/// Implement the Handler trait for the Handle struct.
/// This allows us to call the Handler trait on our Handle struct.
/// This is useful for abstracting over different handler types.
impl<F, Args> Handler<Args> for Handle<F, Args>
where
    F: Handler<Args> + Clone + Send + Sync + 'static,
    Args: Clone + Send + Sync + 'static,
    F::Future: Send,
{
    type Output = F::Output;
    type Future = F::Future;
    fn call(&self, args: Args) -> Self::Future {
        self.0.call(args)
    }
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: FnOnce($($param),*) -> Fut + Clone + Copy + Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
        }
    }
});

// Implement the Handler trait for tuples of different lengths.
factory_tuple! {}
factory_tuple! { T1 }
factory_tuple! { T1 T2 }
factory_tuple! { T1 T2 T3 }
factory_tuple! { T1 T2 T3 T4 }
factory_tuple! { T1 T2 T3 T4 T5 }
factory_tuple! { T1 T2 T3 T4 T5 T6 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 }
factory_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 }
