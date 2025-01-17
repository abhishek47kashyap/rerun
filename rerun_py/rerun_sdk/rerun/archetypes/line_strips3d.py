# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/line_strips3d.fbs".

# You can extend this class by creating a "LineStrips3DExt" class in "line_strips3d_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components, datatypes
from .._baseclasses import (
    Archetype,
)
from ..error_utils import catch_and_log_exceptions

__all__ = ["LineStrips3D"]


@define(str=False, repr=False, init=False)
class LineStrips3D(Archetype):
    """
    **Archetype**: 3D line strips with positions and optional colors, radii, labels, etc.

    Examples
    --------
    ### Many strips:
    ```python
    import rerun as rr

    rr.init("rerun_example_line_strip3d_batch", spawn=True)

    rr.log(
        "strips",
        rr.LineStrips3D(
            [
                [
                    [0, 0, 2],
                    [1, 0, 2],
                    [1, 1, 2],
                    [0, 1, 2],
                ],
                [
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 0, 0],
                    [1, 0, 1],
                    [1, 1, 0],
                    [1, 1, 1],
                    [0, 1, 0],
                    [0, 1, 1],
                ],
            ],
            colors=[[255, 0, 0], [0, 255, 0]],
            radii=[0.025, 0.005],
            labels=["one strip here", "and one strip there"],
        ),
    )
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/1200w.png">
      <img src="https://static.rerun.io/line_strip3d_batch/15e8ff18a6c95a3191acb0eae6eb04adea3b4874/full.png" width="640">
    </picture>
    </center>

    ### Lines with scene & UI radius each:
    ```python
    import rerun as rr

    rr.init("rerun_example_line_strip3d_ui_radius", spawn=True)

    # A blue line with a scene unit radii of 0.01.
    points = [[0, 0, 0], [0, 0, 1], [1, 0, 0], [1, 0, 1]]
    rr.log(
        "scene_unit_line",
        rr.LineStrips3D(
            [points],
            # By default, radii are interpreted as world-space units.
            radii=0.01,
            colors=[0, 0, 255],
        ),
    )

    # A red line with a ui point radii of 5.
    # UI points are independent of zooming in Views, but are sensitive to the application UI scaling.
    # For 100% ui scaling, UI points are equal to pixels.
    points = [[3, 0, 0], [3, 0, 1], [4, 0, 0], [4, 0, 1]]
    rr.log(
        "ui_points_line",
        rr.LineStrips3D(
            [points],
            # rr.Radius.ui_points produces radii that the viewer interprets as given in ui points.
            radii=rr.Radius.ui_points(5.0),
            colors=[255, 0, 0],
        ),
    )
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/1200w.png">
      <img src="https://static.rerun.io/line_strip3d_ui_radius/36b98f47e45747b5a3601511ff39b8d74c61d120/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self: Any,
        strips: components.LineStrip3DArrayLike,
        *,
        radii: datatypes.Float32ArrayLike | None = None,
        colors: datatypes.Rgba32ArrayLike | None = None,
        labels: datatypes.Utf8ArrayLike | None = None,
        show_labels: datatypes.BoolLike | None = None,
        class_ids: datatypes.ClassIdArrayLike | None = None,
    ):
        """
        Create a new instance of the LineStrips3D archetype.

        Parameters
        ----------
        strips:
            All the actual 3D line strips that make up the batch.
        radii:
            Optional radii for the line strips.
        colors:
            Optional colors for the line strips.
        labels:
            Optional text labels for the line strips.

            If there's a single label present, it will be placed at the center of the entity.
            Otherwise, each instance will have its own label.
        show_labels:
            Optional choice of whether the text labels should be shown by default.
        class_ids:
            Optional [`components.ClassId`][rerun.components.ClassId]s for the lines.

            The [`components.ClassId`][rerun.components.ClassId] provides colors and labels if not specified explicitly.

        """

        # You can define your own __init__ function as a member of LineStrips3DExt in line_strips3d_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(
                strips=strips, radii=radii, colors=colors, labels=labels, show_labels=show_labels, class_ids=class_ids
            )
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            strips=None,
            radii=None,
            colors=None,
            labels=None,
            show_labels=None,
            class_ids=None,
        )

    @classmethod
    def _clear(cls) -> LineStrips3D:
        """Produce an empty LineStrips3D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    @classmethod
    def update_fields(
        cls,
        *,
        clear: bool = False,
        strips: components.LineStrip3DArrayLike | None = None,
        radii: datatypes.Float32ArrayLike | None = None,
        colors: datatypes.Rgba32ArrayLike | None = None,
        labels: datatypes.Utf8ArrayLike | None = None,
        show_labels: datatypes.BoolLike | None = None,
        class_ids: datatypes.ClassIdArrayLike | None = None,
    ) -> LineStrips3D:
        """
        Update only some specific fields of a `LineStrips3D`.

        Parameters
        ----------
        clear:
            If true, all unspecified fields will be explicitly cleared.
        strips:
            All the actual 3D line strips that make up the batch.
        radii:
            Optional radii for the line strips.
        colors:
            Optional colors for the line strips.
        labels:
            Optional text labels for the line strips.

            If there's a single label present, it will be placed at the center of the entity.
            Otherwise, each instance will have its own label.
        show_labels:
            Optional choice of whether the text labels should be shown by default.
        class_ids:
            Optional [`components.ClassId`][rerun.components.ClassId]s for the lines.

            The [`components.ClassId`][rerun.components.ClassId] provides colors and labels if not specified explicitly.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            kwargs = {
                "strips": strips,
                "radii": radii,
                "colors": colors,
                "labels": labels,
                "show_labels": show_labels,
                "class_ids": class_ids,
            }

            if clear:
                kwargs = {k: v if v is not None else [] for k, v in kwargs.items()}  # type: ignore[misc]

            inst.__attrs_init__(**kwargs)
            return inst

        inst.__attrs_clear__()
        return inst

    @classmethod
    def clear_fields(cls) -> LineStrips3D:
        """Clear all the fields of a `LineStrips3D`."""
        inst = cls.__new__(cls)
        inst.__attrs_init__(
            strips=[],
            radii=[],
            colors=[],
            labels=[],
            show_labels=[],
            class_ids=[],
        )
        return inst

    strips: components.LineStrip3DBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.LineStrip3DBatch._converter,  # type: ignore[misc]
    )
    # All the actual 3D line strips that make up the batch.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    radii: components.RadiusBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.RadiusBatch._converter,  # type: ignore[misc]
    )
    # Optional radii for the line strips.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    colors: components.ColorBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ColorBatch._converter,  # type: ignore[misc]
    )
    # Optional colors for the line strips.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    labels: components.TextBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.TextBatch._converter,  # type: ignore[misc]
    )
    # Optional text labels for the line strips.
    #
    # If there's a single label present, it will be placed at the center of the entity.
    # Otherwise, each instance will have its own label.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    show_labels: components.ShowLabelsBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ShowLabelsBatch._converter,  # type: ignore[misc]
    )
    # Optional choice of whether the text labels should be shown by default.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    class_ids: components.ClassIdBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ClassIdBatch._converter,  # type: ignore[misc]
    )
    # Optional [`components.ClassId`][rerun.components.ClassId]s for the lines.
    #
    # The [`components.ClassId`][rerun.components.ClassId] provides colors and labels if not specified explicitly.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
