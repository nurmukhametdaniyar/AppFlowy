// This file is generated by rust-protobuf 2.25.2. Do not edit
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
//! Generated file from `cell_entities.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct CreateSelectOptionPayload {
    // message fields
    pub cell_identifier: ::protobuf::SingularPtrField<CellIdentifierPayload>,
    pub option_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateSelectOptionPayload {
    fn default() -> &'a CreateSelectOptionPayload {
        <CreateSelectOptionPayload as ::protobuf::Message>::default_instance()
    }
}

impl CreateSelectOptionPayload {
    pub fn new() -> CreateSelectOptionPayload {
        ::std::default::Default::default()
    }

    // .CellIdentifierPayload cell_identifier = 1;


    pub fn get_cell_identifier(&self) -> &CellIdentifierPayload {
        self.cell_identifier.as_ref().unwrap_or_else(|| <CellIdentifierPayload as ::protobuf::Message>::default_instance())
    }
    pub fn clear_cell_identifier(&mut self) {
        self.cell_identifier.clear();
    }

    pub fn has_cell_identifier(&self) -> bool {
        self.cell_identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cell_identifier(&mut self, v: CellIdentifierPayload) {
        self.cell_identifier = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cell_identifier(&mut self) -> &mut CellIdentifierPayload {
        if self.cell_identifier.is_none() {
            self.cell_identifier.set_default();
        }
        self.cell_identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_cell_identifier(&mut self) -> CellIdentifierPayload {
        self.cell_identifier.take().unwrap_or_else(|| CellIdentifierPayload::new())
    }

    // string option_name = 2;


    pub fn get_option_name(&self) -> &str {
        &self.option_name
    }
    pub fn clear_option_name(&mut self) {
        self.option_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_option_name(&mut self, v: ::std::string::String) {
        self.option_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_option_name(&mut self) -> &mut ::std::string::String {
        &mut self.option_name
    }

    // Take field
    pub fn take_option_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.option_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CreateSelectOptionPayload {
    fn is_initialized(&self) -> bool {
        for v in &self.cell_identifier {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cell_identifier)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.option_name)?;
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
        if let Some(ref v) = self.cell_identifier.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.option_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.option_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cell_identifier.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.option_name.is_empty() {
            os.write_string(2, &self.option_name)?;
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

    fn new() -> CreateSelectOptionPayload {
        CreateSelectOptionPayload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CellIdentifierPayload>>(
                "cell_identifier",
                |m: &CreateSelectOptionPayload| { &m.cell_identifier },
                |m: &mut CreateSelectOptionPayload| { &mut m.cell_identifier },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "option_name",
                |m: &CreateSelectOptionPayload| { &m.option_name },
                |m: &mut CreateSelectOptionPayload| { &mut m.option_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateSelectOptionPayload>(
                "CreateSelectOptionPayload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateSelectOptionPayload {
        static instance: ::protobuf::rt::LazyV2<CreateSelectOptionPayload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateSelectOptionPayload::new)
    }
}

impl ::protobuf::Clear for CreateSelectOptionPayload {
    fn clear(&mut self) {
        self.cell_identifier.clear();
        self.option_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateSelectOptionPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateSelectOptionPayload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CellIdentifierPayload {
    // message fields
    pub grid_id: ::std::string::String,
    pub field_id: ::std::string::String,
    pub row_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CellIdentifierPayload {
    fn default() -> &'a CellIdentifierPayload {
        <CellIdentifierPayload as ::protobuf::Message>::default_instance()
    }
}

impl CellIdentifierPayload {
    pub fn new() -> CellIdentifierPayload {
        ::std::default::Default::default()
    }

    // string grid_id = 1;


    pub fn get_grid_id(&self) -> &str {
        &self.grid_id
    }
    pub fn clear_grid_id(&mut self) {
        self.grid_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_grid_id(&mut self, v: ::std::string::String) {
        self.grid_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grid_id(&mut self) -> &mut ::std::string::String {
        &mut self.grid_id
    }

    // Take field
    pub fn take_grid_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.grid_id, ::std::string::String::new())
    }

    // string field_id = 2;


    pub fn get_field_id(&self) -> &str {
        &self.field_id
    }
    pub fn clear_field_id(&mut self) {
        self.field_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_id(&mut self, v: ::std::string::String) {
        self.field_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_id(&mut self) -> &mut ::std::string::String {
        &mut self.field_id
    }

    // Take field
    pub fn take_field_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_id, ::std::string::String::new())
    }

    // string row_id = 3;


    pub fn get_row_id(&self) -> &str {
        &self.row_id
    }
    pub fn clear_row_id(&mut self) {
        self.row_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_row_id(&mut self, v: ::std::string::String) {
        self.row_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_id(&mut self) -> &mut ::std::string::String {
        &mut self.row_id
    }

    // Take field
    pub fn take_row_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.row_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CellIdentifierPayload {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.grid_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.row_id)?;
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
        if !self.grid_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.grid_id);
        }
        if !self.field_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_id);
        }
        if !self.row_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.row_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.grid_id.is_empty() {
            os.write_string(1, &self.grid_id)?;
        }
        if !self.field_id.is_empty() {
            os.write_string(2, &self.field_id)?;
        }
        if !self.row_id.is_empty() {
            os.write_string(3, &self.row_id)?;
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

    fn new() -> CellIdentifierPayload {
        CellIdentifierPayload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "grid_id",
                |m: &CellIdentifierPayload| { &m.grid_id },
                |m: &mut CellIdentifierPayload| { &mut m.grid_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "field_id",
                |m: &CellIdentifierPayload| { &m.field_id },
                |m: &mut CellIdentifierPayload| { &mut m.field_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "row_id",
                |m: &CellIdentifierPayload| { &m.row_id },
                |m: &mut CellIdentifierPayload| { &mut m.row_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CellIdentifierPayload>(
                "CellIdentifierPayload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CellIdentifierPayload {
        static instance: ::protobuf::rt::LazyV2<CellIdentifierPayload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CellIdentifierPayload::new)
    }
}

impl ::protobuf::Clear for CellIdentifierPayload {
    fn clear(&mut self) {
        self.grid_id.clear();
        self.field_id.clear();
        self.row_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CellIdentifierPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CellIdentifierPayload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelectOptionName {
    // message fields
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SelectOptionName {
    fn default() -> &'a SelectOptionName {
        <SelectOptionName as ::protobuf::Message>::default_instance()
    }
}

impl SelectOptionName {
    pub fn new() -> SelectOptionName {
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
}

impl ::protobuf::Message for SelectOptionName {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

    fn new() -> SelectOptionName {
        SelectOptionName::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &SelectOptionName| { &m.name },
                |m: &mut SelectOptionName| { &mut m.name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SelectOptionName>(
                "SelectOptionName",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SelectOptionName {
        static instance: ::protobuf::rt::LazyV2<SelectOptionName> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SelectOptionName::new)
    }
}

impl ::protobuf::Clear for SelectOptionName {
    fn clear(&mut self) {
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelectOptionName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectOptionName {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13cell_entities.proto\"}\n\x19CreateSelectOptionPayload\x12?\n\x0fce\
    ll_identifier\x18\x01\x20\x01(\x0b2\x16.CellIdentifierPayloadR\x0ecellId\
    entifier\x12\x1f\n\x0boption_name\x18\x02\x20\x01(\tR\noptionName\"b\n\
    \x15CellIdentifierPayload\x12\x17\n\x07grid_id\x18\x01\x20\x01(\tR\x06gr\
    idId\x12\x19\n\x08field_id\x18\x02\x20\x01(\tR\x07fieldId\x12\x15\n\x06r\
    ow_id\x18\x03\x20\x01(\tR\x05rowId\"&\n\x10SelectOptionName\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04nameb\x06proto3\
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
