// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// The description of a semantic Class.
///
/// If an entity is annotated with a corresponding `ClassId`, rerun will use
/// the attached `AnnotationInfo` to derive labels and colors.
///
/// Keypoints within an annotation class can similarly be annotated with a
/// `KeypointId` in which case we should defer to the label and color for the
/// `AnnotationInfo` specifically associated with the Keypoint.
///
/// Keypoints within the class can also be decorated with skeletal edges.
/// Keypoint-connections are pairs of `KeypointId`s. If an edge is
/// defined, and both keypoints exist within the instance of the class, then the
/// keypoints should be connected with an edge. The edge should be labeled and
/// colored as described by the class's `AnnotationInfo`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassDescription {
    /// The `AnnotationInfo` for the class.
    pub info: crate::datatypes::AnnotationInfo,

    /// The `AnnotationInfo` for all of the keypoints.
    pub keypoint_annotations: Vec<crate::datatypes::AnnotationInfo>,

    /// The connections between keypoints.
    pub keypoint_connections: Vec<crate::datatypes::KeypointPair>,
}

impl<'a> From<ClassDescription> for ::std::borrow::Cow<'a, ClassDescription> {
    #[inline]
    fn from(value: ClassDescription) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a ClassDescription> for ::std::borrow::Cow<'a, ClassDescription> {
    #[inline]
    fn from(value: &'a ClassDescription) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for ClassDescription {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = <Vec<Self::Item<'a>> as IntoIterator>::IntoIter;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.ClassDescription".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "info".to_owned(),
                data_type: <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "keypoint_annotations".to_owned(),
                data_type: DataType::List(Box::new(Field {
                    name: "item".to_owned(),
                    data_type: <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                })),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "keypoint_connections".to_owned(),
                data_type: DataType::List(Box::new(Field {
                    name: "item".to_owned(),
                    data_type: <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                })),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::ClassDescription>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::ClassDescription>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                vec![
                    {
                        let (somes, info): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { info, .. } = &**datum;
                                    info.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let info_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = info_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::AnnotationInfo::try_to_arrow_opt(info, None::<&str>)?
                        }
                    },
                    {
                        let (somes, keypoint_annotations): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self {
                                        keypoint_annotations,
                                        ..
                                    } = &**datum;
                                    keypoint_annotations.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint_annotations_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let keypoint_annotations_inner_data: Vec<_> = keypoint_annotations
                                .iter()
                                .flatten()
                                .flatten()
                                .cloned()
                                .map(Some)
                                .collect();
                            let keypoint_annotations_inner_bitmap: Option<
                                ::arrow2::bitmap::Bitmap,
                            > = None;
                            let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                                keypoint_annotations.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.len()).unwrap_or_default()
                                }),
                            )
                            .unwrap()
                            .into();
                            ListArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }))
                                    .to_logical_type()
                                    .clone()
                                },
                                offsets,
                                {
                                    _ = keypoint_annotations_inner_bitmap;
                                    _ = extension_wrapper;
                                    crate::datatypes::AnnotationInfo::try_to_arrow_opt(
                                        keypoint_annotations_inner_data,
                                        None::<&str>,
                                    )?
                                },
                                keypoint_annotations_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, keypoint_connections): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self {
                                        keypoint_connections,
                                        ..
                                    } = &**datum;
                                    keypoint_connections.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint_connections_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let keypoint_connections_inner_data: Vec<_> = keypoint_connections
                                .iter()
                                .flatten()
                                .flatten()
                                .cloned()
                                .map(Some)
                                .collect();
                            let keypoint_connections_inner_bitmap: Option<
                                ::arrow2::bitmap::Bitmap,
                            > = None;
                            let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                                keypoint_connections.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.len()).unwrap_or_default()
                                }),
                            )
                            .unwrap()
                            .into();
                            ListArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }))
                                    .to_logical_type()
                                    .clone()
                                },
                                offsets,
                                {
                                    _ = keypoint_connections_inner_bitmap;
                                    _ = extension_wrapper;
                                    crate::datatypes::KeypointPair::try_to_arrow_opt(
                                        keypoint_connections_inner_data,
                                        None::<&str>,
                                    )?
                                },
                                keypoint_connections_bitmap,
                            )
                            .boxed()
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![
                            Field {
                                name: "info".to_owned(),
                                data_type: <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "keypoint_annotations".to_owned(),
                                data_type: DataType::List(Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type:
                                        <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                })),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "keypoint_connections".to_owned(),
                                data_type: DataType::List(Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: <crate::datatypes::KeypointPair>::to_arrow_datatype(
                                    ),
                                    is_nullable: false,
                                    metadata: [].into(),
                                })),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                        ]),
                        data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.ClassDescription")?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_fields, data_arrays) = (data.fields(), data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(data_arrays)
                    .collect();
                let info = {
                    if !arrays_by_name.contains_key("info") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            DataType::Struct(vec![
                                Field {
                                    name: "info".to_owned(),
                                    data_type:
                                        <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_annotations".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_connections".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                            ]),
                            "info",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let data = &**arrays_by_name["info"];
                    crate::datatypes::AnnotationInfo::try_from_arrow_opt(data)
                        .with_context("rerun.datatypes.ClassDescription#info")?
                        .into_iter()
                };
                let keypoint_annotations = {
                    if !arrays_by_name.contains_key("keypoint_annotations") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            DataType::Struct(vec![
                                Field {
                                    name: "info".to_owned(),
                                    data_type:
                                        <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_annotations".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_connections".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                            ]),
                            "keypoint_annotations",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let data = &**arrays_by_name["keypoint_annotations"];
                    {
                        let data = data
                            .as_any()
                            .downcast_ref::<::arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                crate::DeserializationError::datatype_mismatch(
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    data.data_type().clone(),
                                )
                            })
                            .with_context(
                                "rerun.datatypes.ClassDescription#keypoint_annotations",
                            )?;
                        if data.is_empty() {
                            Vec::new()
                        } else {
                            let data_inner = {
                                let data_inner = &**data.values();
                                crate::datatypes::AnnotationInfo::try_from_arrow_opt(data_inner)
                                    .with_context(
                                        "rerun.datatypes.ClassDescription#keypoint_annotations",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > data_inner.len() {
                                        return Err(crate::DeserializationError::offset_slice_oob(
                                            (start, end),
                                            data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let keypoint_connections = {
                    if !arrays_by_name.contains_key("keypoint_connections") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            DataType::Struct(vec![
                                Field {
                                    name: "info".to_owned(),
                                    data_type:
                                        <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_annotations".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::AnnotationInfo>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                                Field {
                                    name: "keypoint_connections".to_owned(),
                                    data_type: DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    is_nullable: false,
                                    metadata: [].into(),
                                },
                            ]),
                            "keypoint_connections",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let data = &**arrays_by_name["keypoint_connections"];
                    {
                        let data = data
                            .as_any()
                            .downcast_ref::<::arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                crate::DeserializationError::datatype_mismatch(
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::KeypointPair>::to_arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    data.data_type().clone(),
                                )
                            })
                            .with_context(
                                "rerun.datatypes.ClassDescription#keypoint_connections",
                            )?;
                        if data.is_empty() {
                            Vec::new()
                        } else {
                            let data_inner = {
                                let data_inner = &**data.values();
                                crate::datatypes::KeypointPair::try_from_arrow_opt(data_inner)
                                    .with_context(
                                        "rerun.datatypes.ClassDescription#keypoint_connections",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > data_inner.len() {
                                        return Err(crate::DeserializationError::offset_slice_oob(
                                            (start, end),
                                            data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(info, keypoint_annotations, keypoint_connections),
                    data.validity(),
                )
                .map(|opt| {
                    opt.map(|(info, keypoint_annotations, keypoint_connections)| {
                        Ok(Self {
                            info: info
                                .ok_or_else(crate::DeserializationError::missing_data)
                                .with_context("rerun.datatypes.ClassDescription#info")?,
                            keypoint_annotations: keypoint_annotations
                                .ok_or_else(crate::DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescription#keypoint_annotations",
                                )?,
                            keypoint_connections: keypoint_connections
                                .ok_or_else(crate::DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescription#keypoint_connections",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.ClassDescription")?
            }
        })
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Self::try_from_arrow_opt(data)?.into_iter())
    }

    #[inline]
    fn convert_item_to_opt_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for ClassDescription {}