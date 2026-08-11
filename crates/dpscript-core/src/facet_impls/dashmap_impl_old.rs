use core::hash::BuildHasher;
use core::ptr::NonNull;
use std::hash::{Hash, RandomState};

use dashmap::DashMap;
use facet::{OxPtrConst, OxPtrMut, OxPtrUninit, PtrConst, PtrMut, PtrUninit};

use facet::{
    Def, Facet, IterVTable, MapDef, MapVTable, Shape, ShapeBuilder, Type, TypeNameFn, TypeNameOpts,
    TypeOpsIndirect, TypeParam, UserType, VTableDirect, VTableIndirect, Variance, VarianceDep,
    VarianceDesc,
};

type DashMapIterator<'mem, K, V, S = RandomState> = dashmap::iter::Iter<'mem, K, V, S>;

unsafe extern "C" fn dashmap_init_in_place_with_capacity<
    K: Eq + Hash,
    V,
    S: Default + BuildHasher + Clone,
>(
    uninit: PtrUninit,
    capacity: usize,
) -> PtrMut {
    unsafe {
        uninit.put(DashMap::<K, V, S>::with_capacity_and_hasher(
            capacity,
            S::default(),
        ))
    }
}

unsafe extern "C" fn dashmap_insert<K: Eq + core::hash::Hash, V, S: BuildHasher + Clone>(
    ptr: PtrMut,
    key: PtrMut,
    value: PtrMut,
) {
    let map = unsafe { ptr.as_mut::<DashMap<K, V, S>>() };
    let key = unsafe { key.read::<K>() };
    let value = unsafe { value.read::<V>() };
    map.insert(key, value);
}

unsafe extern "C" fn dashmap_insert_owned_string_key<'a, K: Eq + Hash, V, S: BuildHasher + Clone>(
    ptr: PtrMut,
    key: PtrMut,
    value: PtrMut,
) -> bool
where
    K: Facet<'a>,
{
    if K::SHAPE.id != <String as Facet>::SHAPE.id {
        return false;
    }

    let map = unsafe { ptr.as_mut::<DashMap<String, V, S>>() };
    let key = unsafe { key.read::<String>() };
    let value = unsafe { value.read::<V>() };
    map.insert(key, value);
    true
}

// unsafe extern "C" fn dashmap_insert_borrowed_str_key<'a, K: Eq + Hash, V, S: BuildHasher + Clone>(
//     ptr: PtrMut,
//     key: PtrConst,
//     value: PtrMut,
// ) -> bool
// where
//     K: Facet<'a>,
// {
//     if K::SHAPE.id != <String as Facet>::SHAPE.id {
//         return false;
//     }

//     let map = unsafe { ptr.as_mut::<DashMap<String, V, S>>() };
//     let key = unsafe { String::from(key.get::<str>()) };
//     let value = unsafe { value.read::<V>() };
//     map.insert(key, value);
//     true
// }

unsafe extern "C" fn dashmap_insert_borrowed_str_entry<
    'a,
    K: Eq + Hash,
    V,
    S: BuildHasher + Clone,
>(
    ptr: PtrMut,
    key: PtrConst,
    value: PtrConst,
) -> bool
where
    K: Facet<'a>,
    V: Facet<'a>,
{
    if K::SHAPE.id != <String as Facet>::SHAPE.id || V::SHAPE.id != <String as Facet>::SHAPE.id {
        return false;
    }

    let map = unsafe { ptr.as_mut::<DashMap<String, String, S>>() };
    let key = unsafe { String::from(key.get::<str>()) };
    let value = unsafe { String::from(value.get::<str>()) };
    map.insert(key, value);
    true
}

unsafe extern "C" fn dashmap_len<K: Eq + Hash, V, S: BuildHasher + Clone>(ptr: PtrConst) -> usize {
    unsafe { ptr.get::<DashMap<K, V, S>>().len() }
}

unsafe extern "C" fn dashmap_contains_key<K: Eq + core::hash::Hash, V, S: BuildHasher + Clone>(
    ptr: PtrConst,
    key: PtrConst,
) -> bool {
    unsafe { ptr.get::<DashMap<K, V, S>>().contains_key(key.get()) }
}

unsafe extern "C" fn dashmap_get_value_ptr<K: Eq + core::hash::Hash, V, S: BuildHasher + Clone>(
    ptr: PtrConst,
    key: PtrConst,
) -> *const u8 {
    unsafe {
        ptr.get::<DashMap<K, V, S>>()
            .get(key.get())
            .map_or(core::ptr::null(), |v| {
                NonNull::from(v.value()).as_ptr() as *const u8
            })
    }
}

/// Build a DashMap from a contiguous slice of (K, V) pairs.
unsafe extern "C" fn dashmap_from_pair_slice<
    K: Eq + core::hash::Hash,
    V,
    S: Default + BuildHasher + Clone,
>(
    uninit: PtrUninit,
    pairs_ptr: *mut u8,
    count: usize,
) -> PtrMut {
    let pairs = pairs_ptr as *mut (K, V);
    let map = DashMap::<K, V, S>::with_capacity_and_hasher(count, S::default());
    for index in 0..count {
        let (key, value) = unsafe { core::ptr::read(pairs.add(index)) };
        map.insert(key, value);
    }
    unsafe { uninit.put(map) }
}

unsafe extern "C" fn dashmap_iter_init<K: Eq + Hash, V, S: BuildHasher + Clone>(
    ptr: PtrConst,
) -> PtrMut {
    unsafe {
        let map = ptr.get::<DashMap<K, V, S>>();
        let iter: DashMapIterator<'_, K, V, S> = map.iter();
        let iter_state = Box::new(iter);
        PtrMut::new(Box::into_raw(iter_state) as *mut u8)
    }
}

unsafe fn dashmap_iter_next<K: Eq + Hash, V>(iter_ptr: PtrMut) -> Option<(PtrConst, PtrConst)> {
    unsafe {
        // SAFETY: We're extending the lifetime from '_ to 'static through a raw pointer cast.
        // This is sound because:
        // 1. The iterator was allocated in dashmap_iter_init and lives until dashmap_iter_dealloc
        // 2. We only return pointers (PtrConst), not references with the extended lifetime
        // 3. The actual lifetime of the data is managed by the DashMap, not this iterator
        let ptr = iter_ptr.as_mut_ptr::<DashMapIterator<'_, K, V>>();
        let state = &mut *ptr;
        state.next().map(|it| {
            let (key, value) = it.pair();

            (
                PtrConst::new(NonNull::from(key).as_ptr()),
                PtrConst::new(NonNull::from(value).as_ptr()),
            )
        })
    }
}

unsafe extern "C" fn dashmap_iter_dealloc<K, V>(iter_ptr: PtrMut) {
    unsafe {
        drop(Box::from_raw(
            iter_ptr.as_ptr::<DashMapIterator<'_, K, V>>() as *mut DashMapIterator<'_, K, V>,
        ));
    }
}

unsafe fn dashmap_drop<K, V, S>(ox: OxPtrMut) {
    unsafe {
        core::ptr::drop_in_place(ox.as_mut::<DashMap<K, V, S>>());
    }
}

unsafe fn dashmap_default<K: Eq + Hash, V, S: Default + BuildHasher + Clone>(
    ox: OxPtrUninit,
) -> bool {
    unsafe { ox.put(DashMap::<K, V, S>::default()) };
    true
}

unsafe fn dashmap_is_truthy<K: Eq + Hash, V, S: BuildHasher + Clone>(ptr: PtrConst) -> bool {
    !unsafe { ptr.get::<DashMap<K, V, S>>().is_empty() }
}

// TODO: Debug, PartialEq, Eq for DashMap, HashSet
unsafe impl<'a, K, V, S> Facet<'a> for DashMap<K, V, S>
where
    K: Facet<'a> + core::cmp::Eq + core::hash::Hash,
    V: Facet<'a>,
    S: 'a + Default + BuildHasher + Clone,
{
    const SHAPE: &'static Shape = &const {
        const fn build_map_vtable<
            'a,
            K: Facet<'a> + Eq + core::hash::Hash,
            V: Facet<'a>,
            S: Default + BuildHasher + Clone,
        >() -> MapVTable {
            MapVTable::builder()
                .init_in_place_with_capacity(dashmap_init_in_place_with_capacity::<K, V, S>)
                .insert(dashmap_insert::<K, V, S>)
                // .insert_borrowed_str_key(Some(dashmap_insert_borrowed_str_key::<K, V, S>))
                .insert_borrowed_str_entry(Some(dashmap_insert_borrowed_str_entry::<K, V, S>))
                .insert_owned_string_key(Some(dashmap_insert_owned_string_key::<K, V, S>))
                .len(dashmap_len::<K, V, S>)
                .contains_key(dashmap_contains_key::<K, V, S>)
                .get_value_ptr(dashmap_get_value_ptr::<K, V, S>)
                .iter_vtable(IterVTable {
                    init_with_value: Some(dashmap_iter_init::<K, V, S>),
                    next: dashmap_iter_next::<K, V>,
                    next_back: None,
                    size_hint: None,
                    dealloc: dashmap_iter_dealloc::<K, V>,
                })
                .from_pair_slice(Some(dashmap_from_pair_slice::<K, V, S>))
                .pair_stride(core::mem::size_of::<(K, V)>())
                .value_offset_in_pair(core::mem::offset_of!((K, V), 1))
                .build()
        }

        const fn build_type_name<'a, K: Facet<'a>, V: Facet<'a>>() -> TypeNameFn {
            fn type_name_impl<'a, K: Facet<'a>, V: Facet<'a>>(
                _shape: &Shape,
                f: &mut core::fmt::Formatter<'_>,
                opts: TypeNameOpts,
            ) -> core::fmt::Result {
                write!(f, "DashMap")?;
                if let Some(opts) = opts.for_children() {
                    write!(f, "<")?;
                    K::SHAPE.write_type_name(f, opts)?;
                    write!(f, ", ")?;
                    V::SHAPE.write_type_name(f, opts)?;
                    write!(f, ">")?;
                } else {
                    write!(f, "<…>")?;
                }
                Ok(())
            }
            type_name_impl::<K, V>
        }

        ShapeBuilder::for_sized::<Self>("DashMap")
            .module_path("dashmap")
            .type_name(build_type_name::<K, V>())
            .ty(Type::User(UserType::Opaque))
            .def(Def::Map(MapDef {
                vtable: &const { build_map_vtable::<K, V, S>() },
                k: K::SHAPE,
                v: V::SHAPE,
            }))
            .type_params(&[
                TypeParam {
                    name: "K",
                    shape: K::SHAPE,
                },
                TypeParam {
                    name: "V",
                    shape: V::SHAPE,
                },
            ])
            // DashMap<K, V> combines K and V variances
            .variance(VarianceDesc {
                base: Variance::Bivariant,
                deps: &const {
                    [
                        VarianceDep::covariant(K::SHAPE),
                        VarianceDep::covariant(V::SHAPE),
                    ]
                },
            })
            .vtable_indirect(&VTableIndirect::EMPTY)
            .type_ops_indirect(
                &const {
                    TypeOpsIndirect {
                        drop_in_place: dashmap_drop::<K, V, S>,
                        default_in_place: Some(dashmap_default::<K, V, S>),
                        clone_into: None,
                        is_truthy: Some(dashmap_is_truthy::<K, V, S>),
                    }
                },
            )
            .build()
    };
}
