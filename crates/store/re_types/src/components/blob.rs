// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/blob.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A binary blob of data.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Blob(pub ::re_types_core::ArrowBuffer<u8>);

impl ::re_types_core::SizeBytes for Blob {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <::re_types_core::ArrowBuffer<u8>>::is_pod()
    }
}

impl From<::re_types_core::ArrowBuffer<u8>> for Blob {
    #[inline]
    fn from(data: ::re_types_core::ArrowBuffer<u8>) -> Self {
        Self(data)
    }
}

impl From<Blob> for ::re_types_core::ArrowBuffer<u8> {
    #[inline]
    fn from(value: Blob) -> Self {
        value.0
    }
}

impl std::ops::Deref for Blob {
    type Target = ::re_types_core::ArrowBuffer<u8>;

    #[inline]
    fn deref(&self) -> &::re_types_core::ArrowBuffer<u8> {
        &self.0
    }
}

impl std::ops::DerefMut for Blob {
    #[inline]
    fn deref_mut(&mut self) -> &mut ::re_types_core::ArrowBuffer<u8> {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(Blob);

impl ::re_types_core::Loggable for Blob {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Blob".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::List(std::sync::Arc::new(Field::new(
            "item",
            DataType::UInt8,
            false,
        )))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map_or(0, |datum| datum.num_instances())),
                )?
                .into();
                let data0_inner_data: Buffer<_> = data0
                    .iter()
                    .flatten()
                    .map(|b| b.as_slice())
                    .collect::<Vec<_>>()
                    .concat()
                    .into();
                let data0_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                ListArray::try_new(
                    Self::arrow_datatype(),
                    offsets,
                    PrimitiveArray::new(DataType::UInt8, data0_inner_data, data0_inner_bitmap)
                        .boxed(),
                    data0_bitmap,
                )?
                .boxed()
            }
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::ListArray<i32>>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.components.Blob#data")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let arrow_data_inner = {
                    let arrow_data_inner = &**arrow_data.values();
                    arrow_data_inner
                        .as_any()
                        .downcast_ref::<UInt8Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt8;
                            let actual = arrow_data_inner.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.components.Blob#data")?
                        .values()
                };
                let offsets = arrow_data.offsets();
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    offsets.iter().zip(offsets.lengths()),
                    arrow_data.validity(),
                )
                .map(|elem| {
                    elem.map(|(start, len)| {
                        let start = *start as usize;
                        let end = start + len;
                        if end > arrow_data_inner.len() {
                            return Err(DeserializationError::offset_slice_oob(
                                (start, end),
                                arrow_data_inner.len(),
                            ));
                        }

                        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                        let data = unsafe {
                            arrow_data_inner
                                .clone()
                                .sliced_unchecked(start, end - start)
                        };
                        let data = ::re_types_core::ArrowBuffer::from(data);
                        Ok(data)
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| v.ok_or_else(DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.components.Blob#data")
        .with_context("rerun.components.Blob")?)
    }
}