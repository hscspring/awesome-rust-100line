#![doc = include_str!("../README.md")]

/// Create a vector as a constant. Will return as a `ManuallyDrop<Vec<T>>`. 
/// Attempting to drop the inner vec will result in heap corruption, and 
/// attempting to mutate will result in an access violation. Clone if you need
/// a mutable copy. 
#[macro_export]
macro_rules! vec_const
{
    ($t:ty, $($items:expr),*) => 
    {
        {
            std::mem::ManuallyDrop::new
            (
                $crate::vec_const_unsafe!($t, $($items),*)
            )
        }
    }
}

/// Like `vec_const`, but without the safety of the ManuallyDrop wrapper. Can 
/// easily cause catastrophic failure if misused.
#[macro_export]
macro_rules! vec_const_unsafe
{
    ($t:ty, $($items:expr),*) => 
    {
        {
            const FACADE: $crate::VecFacade<$t> = 
            {
                let raw = unsafe {std::mem::transmute::<&[$t], (*const $t, usize)>(&[$($items),*])};
                $crate::VecFacade { ptr: raw.0, cap: raw.1, len: raw.1 }
            };

            std::mem::transmute::<$crate::VecFacade<$t>, Vec<$t>>(FACADE)
        }
    }
}

// Don't try this at home
#[allow(dead_code)]
pub struct VecFacade<T>
{
    pub ptr: *const T,
    pub cap: usize,
    pub len: usize
}

#[cfg(test)]
mod tests 
{
    use std::mem::ManuallyDrop;
    
    #[derive(PartialEq, Clone, Debug, Default)]
    pub struct AThing(u8, &'static str);

    const TEST: ManuallyDrop<Vec<AThing>> = unsafe 
    {
        vec_const!(AThing, AThing(5, "wow"), AThing(2, "cool"))
    };

    #[test]
    fn test() 
    {
        assert_eq!(*TEST, vec!(AThing(5, "wow"), AThing(2, "cool")));
    }

    #[test]
    fn test_2() 
    {
        for item in TEST.clone().iter_mut()
        {
            item.0 = 99;
            item.1 = "weird";
            assert_eq!(item, &AThing(99, "weird"));
        }
    }
}

