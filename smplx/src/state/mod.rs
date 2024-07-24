/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::state::*;

pub(crate) mod state;

pub(crate) mod prelude {
    pub use super::state::*;
}

pub trait StateBase<T> {
    type Cls; // the **type** of the state
    type Obj; // the **type** of the data

    fn data(&self) -> &T;
    fn is_state<Q2: 'static>(&self) -> bool where Self::Cls: 'static;
}

pub trait Stateful<T>: StateBase<T> {
    fn state(&self) -> &Self::Cls;
    fn data(&self) -> &Self::Obj;
}


impl<T, Q> StateBase<T> for State<T, Q> {
    type Cls = Q;
    type Obj = T;

    fn data(&self) -> &T {
        &self.data
    }

    fn is_state<Q2: 'static>(&self) -> bool where Q: 'static {
        use core::any::TypeId;
        TypeId::of::<Q>() == TypeId::of::<Q2>()
    }
}