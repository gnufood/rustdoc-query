//! Pure private-detail row conversions.

use crate::contract::generated::{
    AssociatedConst, AssociatedType, Field, Method, Variant, VariantKind,
};
use crate::query::{AssocTypeRow, ConstRow, FieldRow, MethodRow, VariantRow};

pub(super) fn field(row: FieldRow) -> Field {
    Field {
        name: row.name,
        type_: row.type_,
    }
}

pub(super) fn variant(row: VariantRow) -> Variant {
    Variant {
        name: row.name,
        kind: match row.kind {
            rustdoc_types::VariantKind::Plain => VariantKind::Unit,
            rustdoc_types::VariantKind::Tuple(_) => VariantKind::Tuple,
            rustdoc_types::VariantKind::Struct { .. } => VariantKind::Struct,
        },
        fields: row.fields.into_iter().map(field).collect(),
    }
}

pub(super) fn method(row: MethodRow) -> Method {
    Method {
        name: row.name,
        signature: row.signature,
        impl_constraint: row.impl_constraint,
    }
}

pub(super) fn associated_type(row: AssocTypeRow) -> AssociatedType {
    AssociatedType {
        name: row.name,
        bounds: row.bounds,
        default: row.default,
    }
}

pub(super) fn associated_const(row: ConstRow) -> AssociatedConst {
    AssociatedConst {
        name: row.name,
        type_: row.type_,
    }
}
