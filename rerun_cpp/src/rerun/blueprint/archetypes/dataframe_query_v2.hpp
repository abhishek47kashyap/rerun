// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/dataframe_query.fbs".

#pragma once

#include "../../blueprint/components/apply_latest_at.hpp"
#include "../../blueprint/components/filter_by_event.hpp"
#include "../../blueprint/components/filter_by_range.hpp"
#include "../../blueprint/components/selected_columns.hpp"
#include "../../blueprint/components/timeline_name.hpp"
#include "../../collection.hpp"
#include "../../compiler_utils.hpp"
#include "../../component_batch.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: The query for the dataframe view.
    struct DataframeQueryV2 {
        /// The timeline for this query.
        ///
        /// If unset, the timeline currently active on the time panel is used.
        std::optional<rerun::blueprint::components::TimelineName> timeline;

        /// If provided, only rows whose timestamp is within this range will be shown.
        ///
        /// Note: will be unset as soon as `timeline` is changed.
        std::optional<rerun::blueprint::components::FilterByRange> filter_by_range;

        /// If provided, only show rows which contains a logged event for the specified component.
        std::optional<rerun::blueprint::components::FilterByEvent> filter_by_event;

        /// Should empty cells be filled with latest-at queries?
        std::optional<rerun::blueprint::components::ApplyLatestAt> apply_latest_at;

        /// Selected columns. If unset, all columns are selected.
        std::optional<rerun::blueprint::components::SelectedColumns> select;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.DataframeQueryV2Indicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        DataframeQueryV2() = default;
        DataframeQueryV2(DataframeQueryV2&& other) = default;

        /// The timeline for this query.
        ///
        /// If unset, the timeline currently active on the time panel is used.
        DataframeQueryV2 with_timeline(rerun::blueprint::components::TimelineName _timeline) && {
            timeline = std::move(_timeline);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// If provided, only rows whose timestamp is within this range will be shown.
        ///
        /// Note: will be unset as soon as `timeline` is changed.
        DataframeQueryV2 with_filter_by_range(
            rerun::blueprint::components::FilterByRange _filter_by_range
        ) && {
            filter_by_range = std::move(_filter_by_range);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// If provided, only show rows which contains a logged event for the specified component.
        DataframeQueryV2 with_filter_by_event(
            rerun::blueprint::components::FilterByEvent _filter_by_event
        ) && {
            filter_by_event = std::move(_filter_by_event);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Should empty cells be filled with latest-at queries?
        DataframeQueryV2 with_apply_latest_at(
            rerun::blueprint::components::ApplyLatestAt _apply_latest_at
        ) && {
            apply_latest_at = std::move(_apply_latest_at);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Selected columns. If unset, all columns are selected.
        DataframeQueryV2 with_select(rerun::blueprint::components::SelectedColumns _select) && {
            select = std::move(_select);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::DataframeQueryV2> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const blueprint::archetypes::DataframeQueryV2& archetype
        );
    };
} // namespace rerun