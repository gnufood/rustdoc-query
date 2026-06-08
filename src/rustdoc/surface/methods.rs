use std::collections::BTreeMap;

use rustdoc_types::{Crate, Id, ItemEnum, ItemKind, Visibility};

use super::Surface;

pub(super) fn add_resolvable_inherent_methods(
    surface: &mut Surface,
    root: &Crate,
    dependencies: &[Crate],
) {
    let parents: Vec<_> = surface
        .iter()
        .filter(|entry| matches!(entry.kind, ItemKind::Struct | ItemKind::Enum))
        .cloned()
        .collect();
    for parent in parents {
        let owner = if parent.dep_idx == 0 {
            root
        } else {
            dependencies
                .get(usize::from(parent.dep_idx - 1))
                .unwrap_or(root)
        };
        let Some(item) = owner.index.get(&parent.id) else {
            continue;
        };
        let implementations = match &item.inner {
            ItemEnum::Struct(value) => &value.impls,
            ItemEnum::Enum(value) => &value.impls,
            _ => continue,
        };
        let mut by_name: BTreeMap<String, Vec<(Id, Id)>> = BTreeMap::new();
        for &implementation in implementations {
            let Some(impl_item) = owner.index.get(&implementation) else {
                continue;
            };
            let ItemEnum::Impl(impl_) = &impl_item.inner else {
                continue;
            };
            if impl_.trait_.is_some() {
                continue;
            }
            for &method in &impl_.items {
                let Some(method_item) = owner.index.get(&method) else {
                    continue;
                };
                if method_item.visibility != Visibility::Public
                    || !matches!(method_item.inner, ItemEnum::Function(_))
                {
                    continue;
                }
                if let Some(name) = &method_item.name {
                    by_name
                        .entry(name.clone())
                        .or_default()
                        .push((implementation, method));
                }
            }
        }
        for (name, methods) in by_name {
            let [(implementation, method)] = methods.as_slice() else {
                continue;
            };
            let mut path = parent.path.clone();
            path.push(name);
            surface.record_inherent_method(
                *method,
                path,
                parent.dep_idx,
                parent.id,
                *implementation,
            );
        }
    }
}
