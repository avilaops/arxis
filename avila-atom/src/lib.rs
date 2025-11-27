//! # avila-atom
//!
//! **Átomos Computacionais - Estruturas de Dados Fundamentais**
//!
//! Esta biblioteca combina as partículas fundamentais do `avila-nucleus`
//! em estruturas de dados úteis, da mesma forma que prótons, nêutrons e
//! elétrons se combinam para formar átomos.
//!
//! ## Estruturas Disponíveis
//!
//! - `Option<T>` - Presença ou ausência de valor
//! - `Result<T, E>` - Sucesso ou erro
//! - `Vec<T>` - Lista dinâmica
//! - `HashMap<K, V>` - Mapa de valores
//! - `String` - Sequência de caracteres UTF-8
//!
//! ## Filosofia
//!
//! Estas estruturas são os "átomos" da computação - elementos estáveis
//! que podem ser combinados infinitamente para criar software complexo.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, collections::BTreeMap};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String, collections::HashMap};

/// Re-export do Option
pub use core::option::Option;

/// Re-export do Result
pub use core::result::Result;

/// Re-export do Vec
#[cfg(feature = "std")]
pub type DynamicList<T> = Vec<T>;

#[cfg(not(feature = "std"))]
pub type DynamicList<T> = Vec<T>;

/// Re-export do HashMap (std) ou BTreeMap (no_std)
#[cfg(feature = "std")]
pub type Map<K, V> = HashMap<K, V>;

#[cfg(not(feature = "std"))]
pub type Map<K, V> = BTreeMap<K, V>;

/// Re-export do String
pub type Text = String;

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Macro para criar um Map facilmente
#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut m = $crate::Map::new();
        $(
            m.insert($key, $value);
        )*
        m
    }};
}

/// Macro para criar um Vec facilmente
#[macro_export]
macro_rules! list {
    ($($item:expr),* $(,)?) => {{
        vec![$($item),*]
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let some: Option<i32> = Option::Some(42);
        let none: Option<i32> = Option::None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_result() {
        let ok: Result<i32, &str> = Result::Ok(42);
        let err: Result<i32, &str> = Result::Err("error");

        assert!(ok.is_ok());
        assert!(err.is_err());
    }

    #[test]
    fn test_vec() {
        let mut v: DynamicList<i32> = DynamicList::new();
        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.len(), 3);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_map() {
        let m = map! {
            "key1" => "value1",
            "key2" => "value2",
        };

        assert_eq!(m.get("key1"), Some(&"value1"));
    }

    #[test]
    fn test_string() {
        let s: Text = Text::from("Hello, Avila!");
        assert_eq!(s.len(), 13);
    }
}
