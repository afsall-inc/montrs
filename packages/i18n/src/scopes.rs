use crate::locale_traits::*;

/// Scope marker trait.
pub trait Scope<L: Locale>: 'static + Clone + Copy + Send + Sync {
    type Keys: LocaleKeys<Locale = L>;
    fn get_keys(locale: L) -> Self::Keys;
}

impl<L: Locale> Scope<L> for L::Keys {
    type Keys = L::Keys;
    fn get_keys(locale: L) -> Self::Keys {
        locale.get_keys()
    }
}

/// A locale scoped to a subset of keys.
#[derive(Debug, Clone, Copy)]
pub struct ScopedLocale<L: Locale> {
    inner: L,
}

impl<L: Locale> ScopedLocale<L> {
    pub fn new(locale: L) -> Self {
        Self { inner: locale }
    }
    pub fn inner(self) -> L {
        self.inner
    }
    pub fn as_str(self) -> &'static str {
        self.inner.as_str()
    }
    pub fn direction(self) -> Direction {
        self.inner.direction()
    }
    pub fn get_keys(self) -> L::Keys {
        self.inner.get_keys()
    }
}

impl<L: Locale + Default> Default for ScopedLocale<L> {
    fn default() -> Self {
        Self::new(L::default())
    }
}
impl<L: Locale> AsRef<str> for ScopedLocale<L> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
impl<L: Locale> AsRef<L> for ScopedLocale<L> {
    fn as_ref(&self) -> &L {
        &self.inner
    }
}
impl<L: Locale> std::fmt::Display for ScopedLocale<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl<L: Locale + PartialEq> PartialEq for ScopedLocale<L> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl<L: Locale + Eq> Eq for ScopedLocale<L> {}
impl<L: Locale + std::hash::Hash> std::hash::Hash for ScopedLocale<L> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
impl<L: Locale + serde::Serialize> serde::Serialize for ScopedLocale<L> {
    fn serialize<Ser: serde::Serializer>(
        &self,
        s: Ser,
    ) -> Result<Ser::Ok, Ser::Error> {
        self.inner.serialize(s)
    }
}
impl<'de, L: Locale + serde::de::DeserializeOwned> serde::de::Deserialize<'de>
    for ScopedLocale<L>
{
    fn deserialize<D: serde::de::Deserializer<'de>>(
        d: D,
    ) -> Result<Self, D::Error> {
        L::deserialize(d).map(Self::new)
    }
}
