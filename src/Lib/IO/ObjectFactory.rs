use crate::Lib::IO::IDBObj::IDBObj;

pub fn create_instance<T>() -> T
where
    T: IDBObj + Default,
{
    T::default()
}
