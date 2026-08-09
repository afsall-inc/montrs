//! Display helpers for formatting components with the `td_string!` macro.

use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
};

pub type DynDisplayFn<'a> = &'a dyn Fn(&mut fmt::Formatter<'_>) -> fmt::Result;

#[derive(Clone, Copy)]
pub struct Attributes<'a>(pub &'a [DynDisplayFn<'a>]);

impl Debug for Attributes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attributes").finish()
    }
}

impl Display for Attributes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for attr in self.0 {
            attr(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Children<'a>(pub DynDisplayFn<'a>);

impl Debug for Children<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Children").finish()
    }
}

impl Display for Children<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

#[doc(hidden)]
pub struct WithAttributes<M>(PhantomData<M>);
#[doc(hidden)]
pub struct WithoutAttributes<M>(PhantomData<M>);
#[doc(hidden)]
pub struct WithChildren<M>(PhantomData<M>);
#[doc(hidden)]
pub struct WithoutChildren;
#[doc(hidden)]
pub struct ChildrenFn;
#[doc(hidden)]
pub struct DisplayChildren;

/// Trait used when interpolating components with `td_string!`.
pub trait DisplayComponent<M> {
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        children: T,
        attrs: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result;
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs: Attributes,
    ) -> fmt::Result;
}

impl<F> DisplayComponent<WithAttributes<WithChildren<DisplayChildren>>> for F
where
    F: Fn(&mut fmt::Formatter<'_>, Children, Attributes) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        children: T,
        attrs: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f, Children(&children), attrs)
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs: Attributes,
    ) -> fmt::Result {
        self(f, Children(&|_| Ok(())), attrs)
    }
}

impl<F> DisplayComponent<WithoutAttributes<WithChildren<DisplayChildren>>> for F
where
    F: Fn(&mut fmt::Formatter<'_>, Children) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        children: T,
        _: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f, Children(&children))
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        _: Attributes,
    ) -> fmt::Result {
        self(f, Children(&|_| Ok(())))
    }
}

impl<F> DisplayComponent<WithAttributes<WithoutChildren>> for F
where
    F: Fn(&mut fmt::Formatter<'_>, Attributes) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        _: T,
        attrs: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f, attrs)
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs: Attributes,
    ) -> fmt::Result {
        self(f, attrs)
    }
}

impl<F> DisplayComponent<WithoutAttributes<WithoutChildren>> for F
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        _: T,
        _: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f)
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        _: Attributes,
    ) -> fmt::Result {
        self(f)
    }
}

impl<F> DisplayComponent<WithChildren<ChildrenFn>> for F
where
    F: Fn(&mut fmt::Formatter<'_>, ChildrenFn, DisplayChildren) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        children: T,
        _: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f, ChildrenFn, DisplayChildren);
        drop(children);
        Ok(())
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        _: Attributes,
    ) -> fmt::Result {
        self(f, ChildrenFn, DisplayChildren)
    }
}

impl<F> DisplayComponent<WithAttributes<WithChildren<ChildrenFn>>> for F
where
    F: Fn(
        &mut fmt::Formatter<'_>,
        ChildrenFn,
        DisplayChildren,
        Attributes,
    ) -> fmt::Result,
{
    fn fmt<T>(
        &self,
        f: &mut fmt::Formatter<'_>,
        children: T,
        attrs: Attributes,
    ) -> fmt::Result
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        self(f, ChildrenFn, DisplayChildren, attrs);
        drop(children);
        Ok(())
    }
    fn fmt_self_closing(
        &self,
        f: &mut fmt::Formatter<'_>,
        attrs: Attributes,
    ) -> fmt::Result {
        self(f, ChildrenFn, DisplayChildren, attrs)
    }
}

/// Trait wrapper that combines a locale-key pair for display formatting.
pub trait LangDisplay {
    type Component;
    fn fmt_component(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<F> LangDisplay for F
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    type Component = Self;
    fn fmt_component(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self(f)
    }
}
