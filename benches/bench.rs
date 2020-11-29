use std::io::Cursor;

use abomonation_derive::Abomonation;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prost::Message;
use serde::{Deserialize, Serialize};

#[allow(dead_code, unused_imports)]
#[path = "../storeddata_generated.rs"]
mod flat;

#[allow(dead_code, unused_imports)]
#[path = "../storeddata_capnp.rs"]
mod storeddata_capnp;

pub mod proto3 {
    include!(concat!(env!("OUT_DIR"), "/proto3.rs"));
}

#[derive(
    Abomonation, Serialize, Deserialize, simd_json_derive::Serialize, simd_json_derive::Deserialize,
)]
pub enum StoredVariants {
    YesNo(bool),
    Small(u8),
    Signy(i64),
    Stringy(String),
}

#[derive(
    Abomonation, Serialize, Deserialize, simd_json_derive::Serialize, simd_json_derive::Deserialize,
)]
pub struct StoredData {
    pub variant: StoredVariants,
    pub opt_bool: Option<bool>,
    pub vec_strs: Vec<String>,
    pub range: std::ops::Range<usize>,
}

fn compare_serde(c: &mut Criterion) {
    let mut group = c.benchmark_group("ser");
    let value = StoredData {
        variant: StoredVariants::Signy(42),
        opt_bool: Some(false),
        vec_strs: vec!["Hello, Rust!".into()],
        range: 0..7878,
    };
    let mut buffer = Vec::with_capacity(4096);
    group.bench_function("sr.json", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            serde_json::to_writer(black_box(&mut buffer), black_box(&value))
        })
    });
    println!("json: {} bytes", buffer.len());
    group.bench_function("de.json", |b| {
        b.iter(|| serde_json::from_slice::<'_, StoredData>(black_box(&buffer)))
    });
    group.bench_function("sr.simd-json", |b| {
        use simd_json_derive::Serialize;
        b.iter(|| {
            black_box(&mut buffer).clear();
            value.json_write(black_box(&mut buffer))
        })
    });
    println!("simd-json: {} bytes", buffer.len());
    let mut abobuf = buffer.clone();
    group.bench_function("de.simd-json", |b| {
        use simd_json::AlignedBuf;
        use simd_json_derive::Deserialize;
        let mut string_buffer = Vec::with_capacity(4096);
        let mut input_buffer = AlignedBuf::with_capacity(4096);
        b.iter(|| {
            abobuf.clone_from(&buffer);
            black_box(StoredData::from_slice_with_buffers(
                black_box(&mut abobuf),
                &mut input_buffer,
                &mut string_buffer,
            ))
            .unwrap();
        })
    });
    println!("simd-json: {} bytes", buffer.len());
    group.bench_function("sr.yaml", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            serde_yaml::to_writer(black_box(&mut buffer), black_box(&value))
        })
    });
    println!("yaml: {} bytes", buffer.len());
    group.bench_function("de.yaml", |b| {
        b.iter(|| serde_yaml::from_slice::<StoredData>(black_box(&buffer)))
    });
    group.bench_function("sr.ron", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            ron::ser::to_writer(black_box(&mut buffer), black_box(&value))
        })
    });
    println!("ron: {} bytes", buffer.len());
    group.bench_function("de.ron", |b| {
        b.iter(|| ron::de::from_bytes::<StoredData>(black_box(&buffer)))
    });
    group.bench_function("sr.bincode", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            bincode::serialize_into(black_box(&mut buffer), black_box(&value))
        })
    });
    println!("bincode: {} bytes", buffer.len());
    group.bench_function("de.bincode", |b| {
        b.iter(|| bincode::deserialize::<'_, StoredData>(black_box(&buffer)))
    });
    group.bench_function("sr.msgpack", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            black_box(&value).serialize(&mut rmp_serde::Serializer::new(black_box(&mut buffer)))
        })
    });
    println!("msgpack: {} bytes", buffer.len());
    group.bench_function("de.msgpack", |b| {
        b.iter(|| rmp_serde::from_read_ref::<'_, _, StoredData>(black_box(&buffer)))
    });
    group.bench_function("sr.cbor", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            serde_cbor::to_writer(black_box(&mut buffer), black_box(&value))
        })
    });
    println!("cbor: {} bytes", buffer.len());
    group.bench_function("de.cbor", |b| {
        b.iter(|| serde_cbor::from_slice::<'_, StoredData>(black_box(&buffer)))
    });
    let mut bytes = [0u8; 512];
    group.bench_function("sr.postcard", |b| {
        b.iter(|| {
            let bytes = &mut bytes;
            black_box(postcard::to_slice(black_box(&value), black_box(bytes))).map(|_| ())
        })
    });
    let bytes = &mut [0u8; 512];
    let num_bytes = postcard::to_slice(black_box(&value), black_box(bytes))
        .unwrap()
        .len() as u64;
    println!("postcard: {} bytes", num_bytes);
    let bytes = &*bytes;
    group.bench_function("de.postcard", |b| {
        b.iter(|| postcard::from_bytes::<'_, StoredData>(black_box(bytes)))
    });
    group.bench_function("ser.flexbuffers", |b| {
        b.iter(|| flexbuffers::to_vec(black_box(&value)))
    });
    let flex = flexbuffers::to_vec(black_box(&value)).unwrap();
    println!("flexbuffers: {} bytes", num_bytes);
    group.bench_function("de.flexbuffers", |b| {
        b.iter(|| flexbuffers::from_slice::<'_, StoredData>(black_box(&flex)))
    });
    let mut fbb = flatbuffers::FlatBufferBuilder::new();
    group.bench_function("sr.flatbuffers", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            fbb.reset();
            let variant = Some(
                flat::Int64::create(&mut fbb, &flat::Int64Args { value: 42 }).as_union_value(),
            );
            let strs = fbb.create_vector_of_strings(&["Hello, Rust"]);
            let ofs = flat::StoredData::create(
                &mut fbb,
                &flat::StoredDataArgs {
                    variant_type: flat::StoredVariants::Int64,
                    variant,
                    opt_bool: false,
                    vec_strs: Some(strs),
                    range: Some(&flat::Range::new(0, 42)),
                },
            );
            flat::finish_stored_data_buffer(&mut fbb, ofs);
            black_box(&mut buffer).extend_from_slice(fbb.finished_data());
        })
    });
    println!("flatbuffers: {} bytes", buffer.len());
    group.bench_function("de.flatbuffers", |b| {
        b.iter(|| {
            //flat::get_root_as_stored_data(black_box(&buffer))
            let sd = flat::get_root_as_stored_data(black_box(&buffer));
            let range = sd.range().unwrap();
            use StoredVariants::*;
            StoredData {
                variant: match sd.variant_type() {
                    flat::StoredVariants::Bool => YesNo(sd.variant_as_bool().unwrap().value()),
                    flat::StoredVariants::Uint8 => Small(sd.variant_as_uint_8().unwrap().value()),
                    flat::StoredVariants::Int64 => Signy(sd.variant_as_int_64().unwrap().value()),
                    flat::StoredVariants::String => {
                        Stringy(sd.variant_as_string().unwrap().value().unwrap().to_owned())
                    }
                    _ => panic!(),
                },
                opt_bool: Some(sd.opt_bool()), //TODO: This isn't actually optional
                vec_strs: sd
                    .vec_strs()
                    .unwrap()
                    .iter()
                    .map(|s| s.into())
                    .collect::<Vec<String>>(),
                range: (range.start() as usize)..(range.end() as usize),
            }
        })
    });

    // Preallocate a buffer that can be reused in all iterations.
    let mut scratch_words = capnp::Word::allocate_zeroed_vec(100);
    let mut allocator = capnp::message::ScratchSpaceHeapAllocator::new(
        capnp::Word::words_to_bytes_mut(&mut scratch_words[..]),
    );
    group.bench_function("sr.capnproto.unpacked", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            let mut message = ::capnp::message::Builder::new(&mut allocator);
            let mut stored_data = message.init_root::<storeddata_capnp::stored_data::Builder>();
            stored_data.reborrow().init_variant().set_signy(42);
            stored_data
                .reborrow()
                .init_vec_strs(1)
                .set(0, "Hello, Rust");
            stored_data.reborrow().init_opt_bool().set_value(false);
            let mut range = stored_data.init_range();
            range.set_start(0);
            range.set_end(42);
            capnp::serialize::write_message(black_box(&mut buffer), &message).unwrap();
        })
    });
    println!("capnproto.unpacked: {} bytes", buffer.len());
    group.bench_function("de.capnproto.unpacked", |b| {
        b.iter(|| {
            let message_reader = capnp::serialize::read_message_from_flat_slice(
                &mut black_box(buffer.as_slice()),
                Default::default(),
            )
            .unwrap();
            let sd = message_reader
                .get_root::<storeddata_capnp::stored_data::Reader>()
                .unwrap();
            let range = sd.get_range().unwrap();
            use StoredVariants::*;
            let variant = match sd.get_variant().which().unwrap() {
                storeddata_capnp::stored_data::variant::YesNo(yes_no) => YesNo(yes_no),
                storeddata_capnp::stored_data::variant::Small(small) => Small(small),
                storeddata_capnp::stored_data::variant::Signy(signy) => Signy(signy),
                storeddata_capnp::stored_data::variant::Stringy(stringy) => {
                    Stringy(stringy.unwrap().into())
                }
            };

            let vec_strs: Vec<String> = sd
                .get_vec_strs()
                .unwrap()
                .into_iter()
                .map(|v| v.unwrap().into())
                .collect();

            let opt_bool = match sd.get_opt_bool().which().unwrap() {
                storeddata_capnp::stored_data::opt_bool::None(()) => None,
                storeddata_capnp::stored_data::opt_bool::Value(b) => Some(b),
            };

            StoredData {
                variant,
                opt_bool,
                vec_strs,
                range: (range.get_start() as usize)..(range.get_end() as usize),
            }
        })
    });
    group.bench_function("sr.capnproto.packed", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            let mut message = ::capnp::message::Builder::new(&mut allocator);
            let mut stored_data = message.init_root::<storeddata_capnp::stored_data::Builder>();
            let mut variant = stored_data.reborrow().init_variant();
            variant.set_signy(42);
            let mut strs = stored_data.reborrow().init_vec_strs(1);
            strs.set(0, "Hello, Rust");
            let mut opt_bool = stored_data.reborrow().init_opt_bool();
            opt_bool.set_value(false);
            let mut range = stored_data.init_range();
            range.set_start(0);
            range.set_end(42);
            capnp::serialize_packed::write_message(black_box(&mut buffer), &message).unwrap();
        })
    });
    println!("capnproto.packed: {} bytes", buffer.len());
    group.bench_function("de.capnproto.packed", |b| {
        b.iter(|| {
            let message_reader = capnp::serialize_packed::read_message(
                black_box(buffer.as_slice()),
                Default::default(),
            )
            .unwrap();
            let sd = message_reader
                .get_root::<storeddata_capnp::stored_data::Reader>()
                .unwrap();
            let range = sd.get_range().unwrap();
            use StoredVariants::*;
            let variant = match sd.get_variant().which().unwrap() {
                storeddata_capnp::stored_data::variant::YesNo(yes_no) => YesNo(yes_no),
                storeddata_capnp::stored_data::variant::Small(small) => Small(small),
                storeddata_capnp::stored_data::variant::Signy(signy) => Signy(signy),
                storeddata_capnp::stored_data::variant::Stringy(stringy) => {
                    Stringy(stringy.unwrap().into())
                }
            };
            let vec_strs: Vec<String> = sd
                .get_vec_strs()
                .unwrap()
                .into_iter()
                .map(|v| v.unwrap().into())
                .collect();

            let opt_bool = match sd.get_opt_bool().which().unwrap() {
                storeddata_capnp::stored_data::opt_bool::None(()) => None,
                storeddata_capnp::stored_data::opt_bool::Value(b) => Some(b),
            };

            StoredData {
                variant,
                opt_bool,
                vec_strs,
                range: (range.get_start() as usize)..(range.get_end() as usize),
            }
        })
    });

    group.bench_function("sr.proto3", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            let mut storeddata = proto3::StoredData::default();
            storeddata.variant = Some(match &value.variant {
                StoredVariants::YesNo(v) => proto3::stored_data::Variant::Yesno(*v),
                StoredVariants::Small(v) => proto3::stored_data::Variant::Small((*v).into()),
                StoredVariants::Signy(v) => proto3::stored_data::Variant::Signy(*v),
                StoredVariants::Stringy(v) => proto3::stored_data::Variant::Stringy(v.clone()),
            });
            storeddata.opt_bool = match value.opt_bool {
                Some(v) => Some(proto3::stored_data::OptBool::Value(v)),
                None => None
            };
            storeddata.vec_strs = value.vec_strs.clone();
            storeddata.range = Some(
                proto3::Range {
                    start: value.range.start as u64,
                    end: value.range.end as u64
                }
            );
            storeddata.encode(black_box(&mut buffer)).unwrap();
        })
    });
    println!("proto3: {} bytes", buffer.len());
    group.bench_function("de.proto3", |b| {
        b.iter(|| {
            let storeddata = proto3::StoredData::decode(&mut Cursor::new(&buffer)).unwrap();
            let variant = match &storeddata.variant.as_ref().unwrap() {
                proto3::stored_data::Variant::Yesno(v) => StoredVariants::YesNo(*v),
                proto3::stored_data::Variant::Small(v) => StoredVariants::Small(*v as u8),
                proto3::stored_data::Variant::Signy(v) => StoredVariants::Signy(*v),
                proto3::stored_data::Variant::Stringy(v) => StoredVariants::Stringy(v.to_string()),
            };
            let opt_bool = match &storeddata.opt_bool.as_ref() {
                Some(proto3::stored_data::OptBool::Value(v)) => Some(*v),
                None => None
            };
            let range = &storeddata.range.as_ref().unwrap();
            StoredData {
                variant: variant,
                opt_bool: opt_bool,
                vec_strs: storeddata.vec_strs,
                range: (range.start as usize)..(range.end as usize)
            }
        })
    });

    group.bench_function("sr.abomonation", |b| {
        b.iter(|| {
            black_box(&mut buffer).clear();
            unsafe { abomonation::encode(black_box(&value), black_box(&mut buffer)) }
        })
    });
    println!("abomonation: {} bytes", buffer.len());
    let mut abobuf = buffer.clone();
    group.bench_function("de.abomonation", |b| {
        b.iter(|| unsafe {
            abobuf.clone_from(&buffer);
            black_box(abomonation::decode::<StoredData>(black_box(&mut abobuf))).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, compare_serde);
criterion_main!(benches);
