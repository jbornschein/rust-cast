// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct CastMessage {
    // message fields
    protocol_version: ::std::option::Option<CastMessage_ProtocolVersion>,
    source_id: ::protobuf::SingularField<::std::string::String>,
    destination_id: ::protobuf::SingularField<::std::string::String>,
    namespace: ::protobuf::SingularField<::std::string::String>,
    payload_type: ::std::option::Option<CastMessage_PayloadType>,
    payload_utf8: ::protobuf::SingularField<::std::string::String>,
    payload_binary: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CastMessage {}

impl CastMessage {
    pub fn new() -> CastMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CastMessage {
        static mut instance: ::protobuf::lazy::Lazy<CastMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CastMessage,
        };
        unsafe {
            instance.get(CastMessage::new)
        }
    }

    // required .extensions.api.cast_channel.CastMessage.ProtocolVersion protocol_version = 1;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: CastMessage_ProtocolVersion) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> CastMessage_ProtocolVersion {
        self.protocol_version.unwrap_or(CastMessage_ProtocolVersion::CASTV2_1_0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<CastMessage_ProtocolVersion> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<CastMessage_ProtocolVersion> {
        &mut self.protocol_version
    }

    // required string source_id = 2;

    pub fn clear_source_id(&mut self) {
        self.source_id.clear();
    }

    pub fn has_source_id(&self) -> bool {
        self.source_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_id(&mut self, v: ::std::string::String) {
        self.source_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_id(&mut self) -> &mut ::std::string::String {
        if self.source_id.is_none() {
            self.source_id.set_default();
        };
        self.source_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_id(&mut self) -> ::std::string::String {
        self.source_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source_id(&self) -> &str {
        match self.source_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_source_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.source_id
    }

    fn mut_source_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.source_id
    }

    // required string destination_id = 3;

    pub fn clear_destination_id(&mut self) {
        self.destination_id.clear();
    }

    pub fn has_destination_id(&self) -> bool {
        self.destination_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destination_id(&mut self, v: ::std::string::String) {
        self.destination_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination_id(&mut self) -> &mut ::std::string::String {
        if self.destination_id.is_none() {
            self.destination_id.set_default();
        };
        self.destination_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_destination_id(&mut self) -> ::std::string::String {
        self.destination_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_destination_id(&self) -> &str {
        match self.destination_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_destination_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.destination_id
    }

    fn mut_destination_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.destination_id
    }

    // required string namespace = 4;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    pub fn has_namespace(&self) -> bool {
        self.namespace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        if self.namespace.is_none() {
            self.namespace.set_default();
        };
        self.namespace.as_mut().unwrap()
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        self.namespace.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        match self.namespace.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_namespace_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.namespace
    }

    // required .extensions.api.cast_channel.CastMessage.PayloadType payload_type = 5;

    pub fn clear_payload_type(&mut self) {
        self.payload_type = ::std::option::Option::None;
    }

    pub fn has_payload_type(&self) -> bool {
        self.payload_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload_type(&mut self, v: CastMessage_PayloadType) {
        self.payload_type = ::std::option::Option::Some(v);
    }

    pub fn get_payload_type(&self) -> CastMessage_PayloadType {
        self.payload_type.unwrap_or(CastMessage_PayloadType::STRING)
    }

    fn get_payload_type_for_reflect(&self) -> &::std::option::Option<CastMessage_PayloadType> {
        &self.payload_type
    }

    fn mut_payload_type_for_reflect(&mut self) -> &mut ::std::option::Option<CastMessage_PayloadType> {
        &mut self.payload_type
    }

    // optional string payload_utf8 = 6;

    pub fn clear_payload_utf8(&mut self) {
        self.payload_utf8.clear();
    }

    pub fn has_payload_utf8(&self) -> bool {
        self.payload_utf8.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload_utf8(&mut self, v: ::std::string::String) {
        self.payload_utf8 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload_utf8(&mut self) -> &mut ::std::string::String {
        if self.payload_utf8.is_none() {
            self.payload_utf8.set_default();
        };
        self.payload_utf8.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload_utf8(&mut self) -> ::std::string::String {
        self.payload_utf8.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_payload_utf8(&self) -> &str {
        match self.payload_utf8.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_payload_utf8_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.payload_utf8
    }

    fn mut_payload_utf8_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.payload_utf8
    }

    // optional bytes payload_binary = 7;

    pub fn clear_payload_binary(&mut self) {
        self.payload_binary.clear();
    }

    pub fn has_payload_binary(&self) -> bool {
        self.payload_binary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload_binary(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload_binary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload_binary(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload_binary.is_none() {
            self.payload_binary.set_default();
        };
        self.payload_binary.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload_binary(&mut self) -> ::std::vec::Vec<u8> {
        self.payload_binary.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload_binary(&self) -> &[u8] {
        match self.payload_binary.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_binary_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload_binary
    }

    fn mut_payload_binary_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload_binary
    }
}

impl ::protobuf::Message for CastMessage {
    fn is_initialized(&self) -> bool {
        if self.protocol_version.is_none() {
            return false;
        };
        if self.source_id.is_none() {
            return false;
        };
        if self.destination_id.is_none() {
            return false;
        };
        if self.namespace.is_none() {
            return false;
        };
        if self.payload_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.destination_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.payload_type = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.payload_utf8)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload_binary)?;
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
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.source_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.destination_id.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.namespace.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.payload_type {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.payload_utf8.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.payload_binary.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol_version {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.source_id.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.destination_id.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.namespace.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.payload_type {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.payload_utf8.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.payload_binary.as_ref() {
            os.write_bytes(7, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CastMessage {
    fn new() -> CastMessage {
        CastMessage::new()
    }
}

impl ::protobuf::Clear for CastMessage {
    fn clear(&mut self) {
        self.clear_protocol_version();
        self.clear_source_id();
        self.clear_destination_id();
        self.clear_namespace();
        self.clear_payload_type();
        self.clear_payload_utf8();
        self.clear_payload_binary();
        self.unknown_fields.clear();
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CastMessage_ProtocolVersion {
    CASTV2_1_0 = 0,
}

impl ::protobuf::ProtobufEnum for CastMessage_ProtocolVersion {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CastMessage_ProtocolVersion> {
        match value {
            0 => ::std::option::Option::Some(CastMessage_ProtocolVersion::CASTV2_1_0),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CastMessage_ProtocolVersion] = &[
            CastMessage_ProtocolVersion::CASTV2_1_0,
        ];
        values
    }
}

impl ::std::marker::Copy for CastMessage_ProtocolVersion {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CastMessage_PayloadType {
    STRING = 0,
    BINARY = 1,
}

impl ::protobuf::ProtobufEnum for CastMessage_PayloadType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CastMessage_PayloadType> {
        match value {
            0 => ::std::option::Option::Some(CastMessage_PayloadType::STRING),
            1 => ::std::option::Option::Some(CastMessage_PayloadType::BINARY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CastMessage_PayloadType] = &[
            CastMessage_PayloadType::STRING,
            CastMessage_PayloadType::BINARY,
        ];
        values
    }
}

impl ::std::marker::Copy for CastMessage_PayloadType {
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthChallenge {
    // message fields
    signature_algorithm: ::std::option::Option<SignatureAlgorithm>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthChallenge {}

impl AuthChallenge {
    pub fn new() -> AuthChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthChallenge {
        static mut instance: ::protobuf::lazy::Lazy<AuthChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthChallenge,
        };
        unsafe {
            instance.get(AuthChallenge::new)
        }
    }

    // optional .extensions.api.cast_channel.SignatureAlgorithm signature_algorithm = 1;

    pub fn clear_signature_algorithm(&mut self) {
        self.signature_algorithm = ::std::option::Option::None;
    }

    pub fn has_signature_algorithm(&self) -> bool {
        self.signature_algorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature_algorithm(&mut self, v: SignatureAlgorithm) {
        self.signature_algorithm = ::std::option::Option::Some(v);
    }

    pub fn get_signature_algorithm(&self) -> SignatureAlgorithm {
        self.signature_algorithm.unwrap_or(SignatureAlgorithm::RSASSA_PKCS1v15)
    }

    fn get_signature_algorithm_for_reflect(&self) -> &::std::option::Option<SignatureAlgorithm> {
        &self.signature_algorithm
    }

    fn mut_signature_algorithm_for_reflect(&mut self) -> &mut ::std::option::Option<SignatureAlgorithm> {
        &mut self.signature_algorithm
    }
}

impl ::protobuf::Message for AuthChallenge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.signature_algorithm = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.signature_algorithm {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signature_algorithm {
            os.write_enum(1, v.value())?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthChallenge {
    fn new() -> AuthChallenge {
        AuthChallenge::new()
    }
}

impl ::protobuf::Clear for AuthChallenge {
    fn clear(&mut self) {
        self.clear_signature_algorithm();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthResponse {
    // message fields
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    client_auth_certificate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    intermediate_certificate: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    signature_algorithm: ::std::option::Option<SignatureAlgorithm>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthResponse {}

impl AuthResponse {
    pub fn new() -> AuthResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthResponse {
        static mut instance: ::protobuf::lazy::Lazy<AuthResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthResponse,
        };
        unsafe {
            instance.get(AuthResponse::new)
        }
    }

    // required bytes signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // required bytes client_auth_certificate = 2;

    pub fn clear_client_auth_certificate(&mut self) {
        self.client_auth_certificate.clear();
    }

    pub fn has_client_auth_certificate(&self) -> bool {
        self.client_auth_certificate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_auth_certificate(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_auth_certificate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_auth_certificate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_auth_certificate.is_none() {
            self.client_auth_certificate.set_default();
        };
        self.client_auth_certificate.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_auth_certificate(&mut self) -> ::std::vec::Vec<u8> {
        self.client_auth_certificate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_auth_certificate(&self) -> &[u8] {
        match self.client_auth_certificate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_client_auth_certificate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.client_auth_certificate
    }

    fn mut_client_auth_certificate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.client_auth_certificate
    }

    // repeated bytes intermediate_certificate = 3;

    pub fn clear_intermediate_certificate(&mut self) {
        self.intermediate_certificate.clear();
    }

    // Param is passed by value, moved
    pub fn set_intermediate_certificate(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.intermediate_certificate = v;
    }

    // Mutable pointer to the field.
    pub fn mut_intermediate_certificate(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.intermediate_certificate
    }

    // Take field
    pub fn take_intermediate_certificate(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.intermediate_certificate, ::protobuf::RepeatedField::new())
    }

    pub fn get_intermediate_certificate(&self) -> &[::std::vec::Vec<u8>] {
        &self.intermediate_certificate
    }

    fn get_intermediate_certificate_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.intermediate_certificate
    }

    fn mut_intermediate_certificate_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.intermediate_certificate
    }

    // optional .extensions.api.cast_channel.SignatureAlgorithm signature_algorithm = 4;

    pub fn clear_signature_algorithm(&mut self) {
        self.signature_algorithm = ::std::option::Option::None;
    }

    pub fn has_signature_algorithm(&self) -> bool {
        self.signature_algorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature_algorithm(&mut self, v: SignatureAlgorithm) {
        self.signature_algorithm = ::std::option::Option::Some(v);
    }

    pub fn get_signature_algorithm(&self) -> SignatureAlgorithm {
        self.signature_algorithm.unwrap_or(SignatureAlgorithm::RSASSA_PKCS1v15)
    }

    fn get_signature_algorithm_for_reflect(&self) -> &::std::option::Option<SignatureAlgorithm> {
        &self.signature_algorithm
    }

    fn mut_signature_algorithm_for_reflect(&mut self) -> &mut ::std::option::Option<SignatureAlgorithm> {
        &mut self.signature_algorithm
    }
}

impl ::protobuf::Message for AuthResponse {
    fn is_initialized(&self) -> bool {
        if self.signature.is_none() {
            return false;
        };
        if self.client_auth_certificate.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_auth_certificate)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.intermediate_certificate)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.signature_algorithm = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.client_auth_certificate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        for value in &self.intermediate_certificate {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if let Some(v) = self.signature_algorithm {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.client_auth_certificate.as_ref() {
            os.write_bytes(2, &v)?;
        };
        for v in &self.intermediate_certificate {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.signature_algorithm {
            os.write_enum(4, v.value())?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthResponse {
    fn new() -> AuthResponse {
        AuthResponse::new()
    }
}

impl ::protobuf::Clear for AuthResponse {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_client_auth_certificate();
        self.clear_intermediate_certificate();
        self.clear_signature_algorithm();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthError {
    // message fields
    error_type: ::std::option::Option<AuthError_ErrorType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthError {}

impl AuthError {
    pub fn new() -> AuthError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthError {
        static mut instance: ::protobuf::lazy::Lazy<AuthError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthError,
        };
        unsafe {
            instance.get(AuthError::new)
        }
    }

    // required .extensions.api.cast_channel.AuthError.ErrorType error_type = 1;

    pub fn clear_error_type(&mut self) {
        self.error_type = ::std::option::Option::None;
    }

    pub fn has_error_type(&self) -> bool {
        self.error_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_type(&mut self, v: AuthError_ErrorType) {
        self.error_type = ::std::option::Option::Some(v);
    }

    pub fn get_error_type(&self) -> AuthError_ErrorType {
        self.error_type.unwrap_or(AuthError_ErrorType::INTERNAL_ERROR)
    }

    fn get_error_type_for_reflect(&self) -> &::std::option::Option<AuthError_ErrorType> {
        &self.error_type
    }

    fn mut_error_type_for_reflect(&mut self) -> &mut ::std::option::Option<AuthError_ErrorType> {
        &mut self.error_type
    }
}

impl ::protobuf::Message for AuthError {
    fn is_initialized(&self) -> bool {
        if self.error_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.error_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.error_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error_type {
            os.write_enum(1, v.value())?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthError {
    fn new() -> AuthError {
        AuthError::new()
    }
}

impl ::protobuf::Clear for AuthError {
    fn clear(&mut self) {
        self.clear_error_type();
        self.unknown_fields.clear();
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AuthError_ErrorType {
    INTERNAL_ERROR = 0,
    NO_TLS = 1,
    SIGNATURE_ALGORITHM_UNAVAILABLE = 2,
}

impl ::protobuf::ProtobufEnum for AuthError_ErrorType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AuthError_ErrorType> {
        match value {
            0 => ::std::option::Option::Some(AuthError_ErrorType::INTERNAL_ERROR),
            1 => ::std::option::Option::Some(AuthError_ErrorType::NO_TLS),
            2 => ::std::option::Option::Some(AuthError_ErrorType::SIGNATURE_ALGORITHM_UNAVAILABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AuthError_ErrorType] = &[
            AuthError_ErrorType::INTERNAL_ERROR,
            AuthError_ErrorType::NO_TLS,
            AuthError_ErrorType::SIGNATURE_ALGORITHM_UNAVAILABLE,
        ];
        values
    }
}

impl ::std::marker::Copy for AuthError_ErrorType {
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct DeviceAuthMessage {
    // message fields
    challenge: ::protobuf::SingularPtrField<AuthChallenge>,
    response: ::protobuf::SingularPtrField<AuthResponse>,
    error: ::protobuf::SingularPtrField<AuthError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceAuthMessage {}

impl DeviceAuthMessage {
    pub fn new() -> DeviceAuthMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceAuthMessage {
        static mut instance: ::protobuf::lazy::Lazy<DeviceAuthMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceAuthMessage,
        };
        unsafe {
            instance.get(DeviceAuthMessage::new)
        }
    }

    // optional .extensions.api.cast_channel.AuthChallenge challenge = 1;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: AuthChallenge) {
        self.challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut AuthChallenge {
        if self.challenge.is_none() {
            self.challenge.set_default();
        };
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> AuthChallenge {
        self.challenge.take().unwrap_or_else(|| AuthChallenge::new())
    }

    pub fn get_challenge(&self) -> &AuthChallenge {
        self.challenge.as_ref().unwrap_or_else(|| AuthChallenge::default_instance())
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<AuthChallenge> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AuthChallenge> {
        &mut self.challenge
    }

    // optional .extensions.api.cast_channel.AuthResponse response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: AuthResponse) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut AuthResponse {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> AuthResponse {
        self.response.take().unwrap_or_else(|| AuthResponse::new())
    }

    pub fn get_response(&self) -> &AuthResponse {
        self.response.as_ref().unwrap_or_else(|| AuthResponse::default_instance())
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularPtrField<AuthResponse> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AuthResponse> {
        &mut self.response
    }

    // optional .extensions.api.cast_channel.AuthError error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: AuthError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut AuthError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> AuthError {
        self.error.take().unwrap_or_else(|| AuthError::new())
    }

    pub fn get_error(&self) -> &AuthError {
        self.error.as_ref().unwrap_or_else(|| AuthError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<AuthError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AuthError> {
        &mut self.error
    }
}

impl ::protobuf::Message for DeviceAuthMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.challenge)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(v) = self.challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.challenge.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.response.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.error.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeviceAuthMessage {
    fn new() -> DeviceAuthMessage {
        DeviceAuthMessage::new()
    }
}

impl ::protobuf::Clear for DeviceAuthMessage {
    fn clear(&mut self) {
        self.clear_challenge();
        self.clear_response();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SignatureAlgorithm {
    UNSPECIFIED = 0,
    RSASSA_PKCS1v15 = 1,
    RSASSA_PSS = 2,
}

impl ::protobuf::ProtobufEnum for SignatureAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SignatureAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(SignatureAlgorithm::UNSPECIFIED),
            1 => ::std::option::Option::Some(SignatureAlgorithm::RSASSA_PKCS1v15),
            2 => ::std::option::Option::Some(SignatureAlgorithm::RSASSA_PSS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SignatureAlgorithm] = &[
            SignatureAlgorithm::UNSPECIFIED,
            SignatureAlgorithm::RSASSA_PKCS1v15,
            SignatureAlgorithm::RSASSA_PSS,
        ];
        values
    }
}

impl ::std::marker::Copy for SignatureAlgorithm {
}
