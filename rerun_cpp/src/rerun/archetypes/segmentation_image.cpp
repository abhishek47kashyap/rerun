// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/segmentation_image.fbs".

#include "segmentation_image.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::SegmentationImage>::serialize(
        const archetypes::SegmentationImage& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(5);

        {
            auto result = ComponentBatch::from_loggable(
                archetype.buffer,
                ComponentDescriptor(
                    "rerun.archetypes.SegmentationImage",
                    "buffer",
                    "rerun.components.ImageBuffer"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto result = ComponentBatch::from_loggable(
                archetype.format,
                ComponentDescriptor(
                    "rerun.archetypes.SegmentationImage",
                    "format",
                    "rerun.components.ImageFormat"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.opacity.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.opacity.value(),
                ComponentDescriptor(
                    "rerun.archetypes.SegmentationImage",
                    "opacity",
                    "rerun.components.Opacity"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.draw_order.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.draw_order.value(),
                ComponentDescriptor(
                    "rerun.archetypes.SegmentationImage",
                    "draw_order",
                    "rerun.components.DrawOrder"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = SegmentationImage::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
