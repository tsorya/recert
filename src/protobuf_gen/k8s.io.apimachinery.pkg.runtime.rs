/// RawExtension is used to hold extensions in external versions.
///
/// To use this, make a field which has RawExtension as its type in your external, versioned
/// struct, and Object in your internal struct. You also need to register your
/// various plugin types.
///
/// // Internal package:
///
/// 	type MyAPIObject struct {
/// 		runtime.TypeMeta `json:",inline"`
/// 		MyPlugin runtime.Object `json:"myPlugin"`
/// 	}
///
/// 	type PluginA struct {
/// 		AOption string `json:"aOption"`
/// 	}
///
/// // External package:
///
/// 	type MyAPIObject struct {
/// 		runtime.TypeMeta `json:",inline"`
/// 		MyPlugin runtime.RawExtension `json:"myPlugin"`
/// 	}
///
/// 	type PluginA struct {
/// 		AOption string `json:"aOption"`
/// 	}
///
/// // On the wire, the JSON will look something like this:
///
/// 	{
/// 		"kind":"MyAPIObject",
/// 		"apiVersion":"v1",
/// 		"myPlugin": {
/// 			"kind":"PluginA",
/// 			"aOption":"foo",
/// 		},
/// 	}
///
/// So what happens? Decode first uses json or yaml to unmarshal the serialized data into
/// your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked.
/// The next step is to copy (using pkg/conversion) into the internal struct. The runtime
/// package's DefaultScheme has conversion functions installed which will unpack the
/// JSON stored in RawExtension, turning it into the correct object type, and storing it
/// in the Object. (TODO: In the case where the object is of an unknown type, a
/// runtime.Unknown object will be created and stored.)
///
/// +k8s:deepcopy-gen=true
/// +protobuf=true
/// +k8s:openapi-gen=true
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawExtension {
    /// Raw is the underlying serialization of this object.
    ///
    /// TODO: Determine how to detect ContentType and ContentEncoding of 'Raw' data.
    #[prost(bytes = "vec", optional, tag = "1")]
    pub raw: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// TypeMeta is shared by all top level objects. The proper way to use it is to inline it in your type,
/// like this:
///
/// 	type MyAwesomeAPIObject struct {
/// 	     runtime.TypeMeta    `json:",inline"`
/// 	     ... // other fields
/// 	}
///
/// func (obj *MyAwesomeAPIObject) SetGroupVersionKind(gvk *metav1.GroupVersionKind) { metav1.UpdateTypeMeta(obj,gvk) }; GroupVersionKind() *GroupVersionKind
///
/// TypeMeta is provided here for convenience. You may use it directly from this package or define
/// your own with the same fields.
///
/// +k8s:deepcopy-gen=false
/// +protobuf=true
/// +k8s:openapi-gen=true
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeMeta {
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
}
/// Unknown allows api objects with unknown types to be passed-through. This can be used
/// to deal with the API objects from a plug-in. Unknown objects still have functioning
/// TypeMeta features-- kind, version, etc.
/// TODO: Make this object have easy access to field based accessors and settors for
/// metadata and field mutatation.
///
/// +k8s:deepcopy-gen=true
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
/// +protobuf=true
/// +k8s:openapi-gen=true
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unknown {
    #[prost(message, optional, tag = "1")]
    pub type_meta: ::core::option::Option<TypeMeta>,
    /// Raw will hold the complete serialized object which couldn't be matched
    /// with a registered type. Most likely, nothing should be done with this
    /// except for passing it through the system.
    #[prost(bytes = "vec", optional, tag = "2")]
    pub raw: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// ContentEncoding is encoding used to encode 'Raw' data.
    /// Unspecified means no encoding.
    #[prost(string, optional, tag = "3")]
    pub content_encoding: ::core::option::Option<::prost::alloc::string::String>,
    /// ContentType  is serialization method used to serialize 'Raw'.
    /// Unspecified means ContentTypeJSON.
    #[prost(string, optional, tag = "4")]
    pub content_type: ::core::option::Option<::prost::alloc::string::String>,
}