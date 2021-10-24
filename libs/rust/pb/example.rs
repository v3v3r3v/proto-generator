// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `protos/example/example.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct ExampleMessage {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub email: ::std::string::String,
    pub last_updated: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub gender: ExampleMessage_Gender,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExampleMessage {
    fn default() -> &'a ExampleMessage {
        <ExampleMessage as ::protobuf::Message>::default_instance()
    }
}

impl ExampleMessage {
    pub fn new() -> ExampleMessage {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int32 id = 2;


    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    // string email = 3;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp last_updated = 4;


    pub fn get_last_updated(&self) -> &::protobuf::well_known_types::Timestamp {
        self.last_updated.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_last_updated(&mut self) {
        self.last_updated.clear();
    }

    pub fn has_last_updated(&self) -> bool {
        self.last_updated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_updated(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.last_updated = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_updated(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.last_updated.is_none() {
            self.last_updated.set_default();
        }
        self.last_updated.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_updated(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.last_updated.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .Example.ExampleMessage.Gender gender = 5;


    pub fn get_gender(&self) -> ExampleMessage_Gender {
        self.gender
    }
    pub fn clear_gender(&mut self) {
        self.gender = ExampleMessage_Gender::NONE;
    }

    // Param is passed by value, moved
    pub fn set_gender(&mut self, v: ExampleMessage_Gender) {
        self.gender = v;
    }
}

impl ::protobuf::Message for ExampleMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.last_updated {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_updated)?;
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.gender, 5, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.email);
        }
        if let Some(ref v) = self.last_updated.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.gender != ExampleMessage_Gender::NONE {
            my_size += ::protobuf::rt::enum_size(5, self.gender);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        }
        if !self.email.is_empty() {
            os.write_string(3, &self.email)?;
        }
        if let Some(ref v) = self.last_updated.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.gender != ExampleMessage_Gender::NONE {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.gender))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ExampleMessage {
        ExampleMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &ExampleMessage| { &m.name },
                |m: &mut ExampleMessage| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "id",
                |m: &ExampleMessage| { &m.id },
                |m: &mut ExampleMessage| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &ExampleMessage| { &m.email },
                |m: &mut ExampleMessage| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "last_updated",
                |m: &ExampleMessage| { &m.last_updated },
                |m: &mut ExampleMessage| { &mut m.last_updated },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExampleMessage_Gender>>(
                "gender",
                |m: &ExampleMessage| { &m.gender },
                |m: &mut ExampleMessage| { &mut m.gender },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExampleMessage>(
                "ExampleMessage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExampleMessage {
        static instance: ::protobuf::rt::LazyV2<ExampleMessage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExampleMessage::new)
    }
}

impl ::protobuf::Clear for ExampleMessage {
    fn clear(&mut self) {
        self.name.clear();
        self.id = 0;
        self.email.clear();
        self.last_updated.clear();
        self.gender = ExampleMessage_Gender::NONE;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExampleMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExampleMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExampleMessage_Gender {
    NONE = 0,
    MALE = 1,
    FEMALE = 2,
}

impl ::protobuf::ProtobufEnum for ExampleMessage_Gender {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExampleMessage_Gender> {
        match value {
            0 => ::std::option::Option::Some(ExampleMessage_Gender::NONE),
            1 => ::std::option::Option::Some(ExampleMessage_Gender::MALE),
            2 => ::std::option::Option::Some(ExampleMessage_Gender::FEMALE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExampleMessage_Gender] = &[
            ExampleMessage_Gender::NONE,
            ExampleMessage_Gender::MALE,
            ExampleMessage_Gender::FEMALE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ExampleMessage_Gender>("ExampleMessage.Gender", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ExampleMessage_Gender {
}

impl ::std::default::Default for ExampleMessage_Gender {
    fn default() -> Self {
        ExampleMessage_Gender::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for ExampleMessage_Gender {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cprotos/example/example.proto\x12\x07Example\x1a\x1fgoogle/protobuf\
    /timestamp.proto\"\xeb\x01\n\x0eExampleMessage\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\x02\x20\x01(\x05R\x02id\x12\x14\
    \n\x05email\x18\x03\x20\x01(\tR\x05email\x12=\n\x0clast_updated\x18\x04\
    \x20\x01(\x0b2\x1a.google.protobuf.TimestampR\x0blastUpdated\x126\n\x06g\
    ender\x18\x05\x20\x01(\x0e2\x1e.Example.ExampleMessage.GenderR\x06gender\
    \"(\n\x06Gender\x12\x08\n\x04NONE\x10\0\x12\x08\n\x04MALE\x10\x01\x12\n\
    \n\x06FEMALE\x10\x02B'Z\x0b/pb/example\xaa\x02\nPb.Example\xca\x02\nPb\\\
    ExampleJ\x91\x05\n\x06\x12\x04\x05\0\x1a\x01\n_\n\x01\x0c\x12\x03\x05\0\
    \x122U*\n\x20Example\x20entity\x20Protocol\x20Buffers\n\x20Author:\x20Zv\
    erev\x20Valeriy\x20<zverevvaleriy@gmail.com>\n\n\x08\n\x01\x02\x12\x03\
    \x07\0\x10\n\x08\n\x01\x08\x12\x03\t\0\"\n\t\n\x02\x08\x0b\x12\x03\t\0\"\
    \n\x08\n\x01\x08\x12\x03\n\0'\n\t\n\x02\x08%\x12\x03\n\0'\n\x08\n\x01\
    \x08\x12\x03\x0b\0%\n\t\n\x02\x08)\x12\x03\x0b\0%\n\t\n\x02\x03\0\x12\
    \x03\r\0)\n\n\n\x02\x04\0\x12\x04\x0f\0\x1a\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x0f\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03\x10\x04\x14\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x10\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x10\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x10\x12\x13\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x11\x04\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x11\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x11\n\x0c\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x11\x0f\x10\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x12\x04\x15\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x12\x04\n\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x12\x0b\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x12\x13\x14\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x13\x04/\n\x0c\n\x05\
    \x04\0\x02\x03\x06\x12\x03\x13\x04\x1d\n\x0c\n\x05\x04\0\x02\x03\x01\x12\
    \x03\x13\x1e*\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x13-.\n\x0c\n\x04\
    \x04\0\x04\0\x12\x04\x14\x04\x18\x05\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\
    \x14\t\x0f\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x15\x08\x11\n\x0e\n\x07\
    \x04\0\x04\0\x02\0\x01\x12\x03\x15\x08\x0c\n\x0e\n\x07\x04\0\x04\0\x02\0\
    \x02\x12\x03\x15\x0f\x10\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03\x16\x08\
    \x11\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x16\x08\x0c\n\x0e\n\x07\
    \x04\0\x04\0\x02\x01\x02\x12\x03\x16\x0f\x10\n\r\n\x06\x04\0\x04\0\x02\
    \x02\x12\x03\x17\x08\x13\n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03\x17\
    \x08\x0e\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x17\x11\x12\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03\x19\x04\x16\n\x0c\n\x05\x04\0\x02\x04\x06\x12\
    \x03\x19\x04\n\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x19\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\x19\x14\x15b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
