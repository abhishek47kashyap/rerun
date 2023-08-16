// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/vector3d.fbs"

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/vec3d.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <utility>

namespace rerun {
    namespace components {
        /// A vector in 3D space.
        struct Vector3D {
            rerun::datatypes::Vec3D vector;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            // Extensions to generated type defined in 'vector3d_ext.cpp'

            /// Construct Vector3D from x/y/z values.
            Vector3D(float x, float y, float z) : vector{x, y, z} {}

            float x() const {
                return vector.x();
            }

            float y() const {
                return vector.y();
            }

            float z() const {
                return vector.z();
            }

          public:
            Vector3D() = default;

            Vector3D(rerun::datatypes::Vec3D _vector) : vector(std::move(_vector)) {}

            Vector3D& operator=(rerun::datatypes::Vec3D _vector) {
                vector = std::move(_vector);
                return *this;
            }

            Vector3D(const float (&arg)[3]) : vector(arg) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::FixedSizeListBuilder>>
                new_arrow_array_builder(arrow::MemoryPool* memory_pool);

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::FixedSizeListBuilder* builder, const Vector3D* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of Vector3D components.
            static arrow::Result<rerun::DataCell> to_data_cell(
                const Vector3D* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun