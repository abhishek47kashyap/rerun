# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/range2d.fbs".

# You can extend this class by creating a "Range2DExt" class in "range2d_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import ComponentBatchMixin
from .range2d_ext import Range2DExt

__all__ = ["Range2D", "Range2DBatch", "Range2DType"]


class Range2D(Range2DExt, datatypes.Range2D):
    """**Component**: An Axis-Aligned Bounding Box in 2D space."""

    # __init__ can be found in range2d_ext.py

    # Note: there are no fields here because Range2D delegates to datatypes.Range2D
    pass


class Range2DType(datatypes.Range2DType):
    _TYPE_NAME: str = "rerun.components.Range2D"


class Range2DBatch(datatypes.Range2DBatch, ComponentBatchMixin):
    _ARROW_TYPE = Range2DType()