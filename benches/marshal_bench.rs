use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustbus::message::Container;
use rustbus::message::DictMap;
use rustbus::message::Param;
use rustbus::wire::marshal::marshal;

const MESSAGE_SIZE: usize = 19;

fn marsh(msg: &rustbus::Message, buf: &mut Vec<u8>) {
    marshal(msg, rustbus::message::ByteOrder::LittleEndian, &[], buf).unwrap();
}

fn make_rustbus_message() -> rustbus::Message {
    let mut params: Vec<Param> = Vec::new();

    let mut dict = DictMap::new();
    dict.insert("A".to_owned().into(), 1234567i32.into());
    dict.insert("B".to_owned().into(), 1234567i32.into());
    dict.insert("C".to_owned().into(), 1234567i32.into());
    dict.insert("D".to_owned().into(), 1234567i32.into());
    dict.insert("E".to_owned().into(), 1234567i32.into());

    use std::convert::TryFrom;
    let dict: Param = Container::try_from(dict).unwrap().into();

    let array: Param = Container::try_from(vec![
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
    ])
    .unwrap()
    .into();

    for _ in 0..MESSAGE_SIZE {
        params.push("TesttestTesttest".to_owned().into());
        params.push(0xFFFFFFFFFFFFFFFFu64.into());
        params.push(
            Container::Struct(vec![
                0xFFFFFFFFFFFFFFFFu64.into(),
                "TesttestTesttest".to_owned().into(),
            ])
            .into(),
        );
        params.push(dict.clone());
        params.push(array.clone());
    }

    let mut msg = rustbus::message_builder::MessageBuilder::new()
        .signal(
            "io.killing.spark".into(),
            "TestSignal".into(),
            "/io/killing/spark".into(),
        )
        .with_params(params)
        .build();
    msg.serial = Some(1);
    msg
}

fn make_dbus_message_parser_message() -> dbus_message_parser::Message {
    let mut signal =
        dbus_message_parser::Message::signal("/io/killing/spark", "io.killing.spark", "TestSignal");

    let dict = dbus_message_parser::Value::Array(
        vec![
            dbus_message_parser::Value::DictEntry(Box::new((
                dbus_message_parser::Value::String("A".to_owned()),
                dbus_message_parser::Value::Int32(1234567i32),
            ))),
            dbus_message_parser::Value::DictEntry(Box::new((
                dbus_message_parser::Value::String("B".to_owned()),
                dbus_message_parser::Value::Int32(1234567i32),
            ))),
            dbus_message_parser::Value::DictEntry(Box::new((
                dbus_message_parser::Value::String("C".to_owned()),
                dbus_message_parser::Value::Int32(1234567i32),
            ))),
            dbus_message_parser::Value::DictEntry(Box::new((
                dbus_message_parser::Value::String("D".to_owned()),
                dbus_message_parser::Value::Int32(1234567i32),
            ))),
            dbus_message_parser::Value::DictEntry(Box::new((
                dbus_message_parser::Value::String("E".to_owned()),
                dbus_message_parser::Value::Int32(1234567i32),
            ))),
        ],
        "{si}".into(),
    );

    let array = dbus_message_parser::Value::Array(
        vec![
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
        ],
        "t".to_owned(),
    );

    for _ in 0..MESSAGE_SIZE {
        signal.add_value(dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64));
        signal.add_value(dbus_message_parser::Value::String(
            "TesttestTesttest".into(),
        ));
        signal.add_value(dbus_message_parser::Value::Struct(vec![
            dbus_message_parser::Value::Uint64(0xFFFFFFFFFFFFFFFFu64),
            dbus_message_parser::Value::String("TesttestTesttest".into()),
        ]));
        signal.add_value(dict.clone());
        signal.add_value(array.clone());
    }
    signal
}

fn make_dbus_pure_message() -> (dbus_pure::proto::MessageHeader<'static>, dbus_pure::proto::Variant<'static>) {
    let header = dbus_pure::proto::MessageHeader {
        r#type: dbus_pure::proto::MessageType::Signal {
            interface: "io.killing.spark".into(),
            member: "TestSignal".into(),
            path: dbus_pure::proto::ObjectPath("/io/killing/spark".into()),
        },
        flags: dbus_pure::proto::message_flags::NO_REPLY_EXPECTED,
        body_len: 0,
        serial: 0,
        fields: (&[][..]).into(),
    };

    let dict = dbus_pure::proto::Variant::Array {
        element_signature: dbus_pure::proto::Signature::DictEntry {
            key: Box::new(dbus_pure::proto::Signature::String),
            value: Box::new(dbus_pure::proto::Signature::I32),
        },
        elements: vec![
            dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String("A".into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(1234567i32)).into(),
            },
            dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String("B".into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(1234567i32)).into(),
            },
            dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String("C".into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(1234567i32)).into(),
            },
            dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String("D".into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(1234567i32)).into(),
            },
            dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String("E".into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(1234567i32)).into(),
            },
        ].into(),
    };

    let array = dbus_pure::proto::Variant::ArrayU64((&[0xFFFFFFFFFFFFFFFFu64; 15][..]).into());

    let mut elements = vec![];

    for _ in 0..MESSAGE_SIZE {
        elements.push(dbus_pure::proto::Variant::U64(0xFFFFFFFFFFFFFFFFu64));
        elements.push(dbus_pure::proto::Variant::String("TesttestTesttest".into()));
        elements.push(dbus_pure::proto::Variant::Struct {
            fields: vec![
                dbus_pure::proto::Variant::U64(0xFFFFFFFFFFFFFFFFu64),
                dbus_pure::proto::Variant::String("TesttestTesttest".into()),
            ].into(),
        });
        elements.push(dict.clone());
        elements.push(array.clone());
    }

    let body = dbus_pure::proto::Variant::Tuple { elements: elements.into() };

    (header, body)
}

fn make_dbusrs_message() -> dbus::Message {
    let mut msg = dbus::message::Message::signal(
        &dbus::strings::Path::from("/io/killing/spark"),
        &dbus::strings::Interface::from("io.killing.spark"),
        &dbus::strings::Member::from("TestSignal"),
    );

    let dict = dbus::arg::Dict::new(
        vec![
            ("A".to_owned(), 1234567i32),
            ("B".to_owned(), 1234567i32),
            ("C".to_owned(), 1234567i32),
            ("D".to_owned(), 1234567i32),
            ("E".to_owned(), 1234567i32),
        ]
        .into_iter(),
    );

    let array = vec![
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
    ];

    for _ in 0..MESSAGE_SIZE {
        msg = msg.append3(
            "TesttestTesttest",
            0xFFFFFFFFFFFFFFFFu64,
            (0xFFFFFFFFFFFFFFFFu64, "TesttestTesttest"),
        );
        msg = msg.append2(&dict, &array);
    }
    msg
}

fn make_zvariant_message() -> zvariant::Structure {
    let mut body = zvariant::Structure::new();
    let mut struct_field = zvariant::Structure::new();

    let mut map = std::collections::HashMap::new();
    map.insert("A".to_owned(), 1234567i32);
    map.insert("B".to_owned(), 1234567i32);
    map.insert("C".to_owned(), 1234567i32);
    map.insert("D".to_owned(), 1234567i32);
    map.insert("E".to_owned(), 1234567i32);

    let dict = zvariant::Dict::from(map);
    use std::convert::TryFrom;
    let dict_arr = zvariant::Array::try_from(dict).unwrap();

    let array = zvariant::Array::from(vec![
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
    ]);

    struct_field = struct_field.add_field(0xFFFFFFFFFFFFFFFFu64);
    struct_field = struct_field.add_field("TesttestTesttest");

    for _ in 0..MESSAGE_SIZE {
        body = body.add_field("TesttestTesttest");
        body = body.add_field(0xFFFFFFFFFFFFFFFFu64);
        body = body.add_field(struct_field.clone());
        // TODO is this really the most efficient way?
        body = body.add_field(dict_arr.clone());
        body = body.add_field(array.clone());
    }

    body
}

fn make_dbus_bytestream_message() -> dbus_bytestream::message::Message {
    let mut msg = dbus_bytestream::message::create_signal(
        "io.killing.spark",
        "TestSignal",
        "/io/killing/spark",
    );

    let mut map = std::collections::HashMap::new();
    map.insert(
        dbus_serialize::types::BasicValue::String("A".into()),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
            1234567i32,
        )),
    );
    map.insert(
        dbus_serialize::types::BasicValue::String("B".into()),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
            1234567i32,
        )),
    );
    map.insert(
        dbus_serialize::types::BasicValue::String("C".into()),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
            1234567i32,
        )),
    );
    map.insert(
        dbus_serialize::types::BasicValue::String("D".into()),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
            1234567i32,
        )),
    );
    map.insert(
        dbus_serialize::types::BasicValue::String("E".into()),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
            1234567i32,
        )),
    );

    let array = vec![
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
        dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
            0xFFFFFFFFFFFFFFFFu64,
        )),
    ];

    for _ in 0..MESSAGE_SIZE {
        msg = msg.add_arg(&"TesttestTesttest");
        msg = msg.add_arg(&0xFFFFFFFFFFFFFFFFu64);
        msg = msg.add_arg(&dbus_serialize::types::Struct {
            objects: vec![
                dbus_serialize::types::Value::BasicValue(
                    dbus_serialize::types::BasicValue::Uint64(0xFFFFFFFFFFFFFFFFu64),
                ),
                dbus_serialize::types::Value::BasicValue(
                    dbus_serialize::types::BasicValue::String("TesttestTesttest".into()),
                ),
            ],
            signature: dbus_serialize::types::Signature("ts".to_owned()),
        });
        msg = msg.add_arg(&map);
        msg = msg.add_arg(&array);
    }

    msg
}

fn criterion_benchmark(c: &mut Criterion) {
    //
    // This tests only marshalling speed
    // I think that libdbus and by that dbus-rs marshal values as they are added
    // so just creating the message is equivalent to create+marshal in rustbus?
    //
    c.bench_function("marshal_rustbus", |b| {
        b.iter(|| {
            let msg = make_rustbus_message();
            let mut buf = Vec::new();
            buf.clear();
            marsh(black_box(&msg), &mut buf);
            return msg;
        })
    });
    c.bench_function("marshal_dbusrs", |b| {
        b.iter(|| {
            let msg = make_dbusrs_message();
            return msg;
        })
    });
    c.bench_function("marshal_dbus_bytestream", |b| {
        b.iter(|| {
            let msg = make_dbus_bytestream_message();
            let mut buf = Vec::new();

            use dbus_bytestream::marshal::Marshal;
            msg.dbus_encode(&mut buf);
            return msg;
        })
    });
    c.bench_function("marshal_dbus_message_parser", |b| {
        b.iter(|| {
            let signal = make_dbus_message_parser_message();
            let mut buffer = bytes::BytesMut::new();
            signal.encode(&mut buffer).unwrap();
            return signal;
        })
    });
    c.bench_function("marshal_dbus_pure", |b| {
        b.iter(|| {
            let (mut header, body) = make_dbus_pure_message();
            let mut buf = Vec::new();
            dbus_pure::proto::serialize_message(&mut header, Some(&body), &mut buf, dbus_pure::proto::Endianness::Little).unwrap();
            return (header, body);
        })
    });
    c.bench_function("marshal_zvariant", |b| {
        b.iter(|| {
            let body = make_zvariant_message();
            let msg = zbus::Message::method(
                Some("io.killing.spark"),
                "/io/killing/spark",
                Some("io.killing.spark"),
                "TestSignal",
                Some(body),
            )
            .unwrap();
            return msg;
        })
    });

    //
    // This tests the flow of:
    // 1. Connect to the session bus (which needs a hello message, which is implicit for dbus-rs)
    // 2. Create a signal message
    // 3. Send the signal to the bus
    //
    c.bench_function("send_rustbus", |b| {
        b.iter(|| {
            let mut rustbus_con = rustbus::client_conn::RpcConn::new(
                rustbus::client_conn::Conn::connect_to_bus(
                    rustbus::get_session_bus_path().unwrap(),
                    false,
                )
                .unwrap(),
            );
            let msg = make_rustbus_message();
            let serial = rustbus_con
                .send_message(rustbus::standard_messages::hello(), None)
                .unwrap()
                .serial
                .unwrap();
            let _name_resp = rustbus_con.wait_response(serial, None).unwrap();
            let _serial = rustbus_con.send_message(msg, None).unwrap();
        })
    });
    c.bench_function("send_dbusrs", |b| {
        b.iter(|| {
            use dbus::channel::Sender;
            let conn = dbus::blocking::Connection::new_session().unwrap();
            let msg = make_dbusrs_message();
            conn.send(msg).unwrap();
        })
    });
    c.bench_function("send_dbus_bytestream", |b| {
        b.iter(|| {
            let conn = dbus_bytestream::connection::Connection::connect_session().unwrap();
            let msg = make_dbus_bytestream_message();
            conn.send(msg).unwrap();
        })
    });
    c.bench_function("send_dbus_pure", |b| {
        b.iter(|| {
            let connection =
                dbus_pure::Connection::new(
                    dbus_pure::BusPath::System,
                    dbus_pure::SaslAuthType::Uid,
                ).unwrap();
            let mut dbus_client = dbus_pure::Client::new(connection).unwrap();
            let (mut header, body) = make_dbus_pure_message();
            let _ = dbus_client.send(&mut header, Some(&body)).unwrap();
        })
    });
    // currently this does a lot of println so it is not a fair comparison
    //c.bench_function("send_zvariant", |b| {
    //    b.iter(|| {
    //        let mut con = zbus::Connection::new_session().unwrap();
    //        let body = make_zvariant_message();
    //        // this crate does not yet support signals so we send a call to a nonexistent service
    //        assert!(con
    //            .call_method(
    //                Some("io.killing.spark"),
    //                "/io/killing/spark",
    //                Some("io.killing.spark"),
    //                "TestSignal",
    //                Some(body),
    //            )
    //            .is_err());
    //    })
    //});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
