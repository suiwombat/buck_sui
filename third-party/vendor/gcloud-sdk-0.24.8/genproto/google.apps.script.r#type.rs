// This file is @generated by prost-build.
/// The widget subset used by an add-on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOnWidgetSet {
    /// The list of widgets used in an add-on.
    #[prost(enumeration = "add_on_widget_set::WidgetType", repeated, tag = "1")]
    pub used_widgets: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `AddOnWidgetSet`.
pub mod add_on_widget_set {
    /// The Widget type. DEFAULT is the basic widget set.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WidgetType {
        /// The default widget set.
        Unspecified = 0,
        /// The date picker.
        DatePicker = 1,
        /// Styled buttons include filled buttons and disabled buttons.
        StyledButtons = 2,
        /// Persistent forms allow persisting form values during actions.
        PersistentForms = 3,
        /// Fixed footer in card.
        FixedFooter = 4,
        /// Update the subject and recipients of a draft.
        UpdateSubjectAndRecipients = 5,
        /// The grid widget.
        GridWidget = 6,
        /// A Gmail add-on action that applies to the addon compose UI.
        AddonComposeUiAction = 7,
    }
    impl WidgetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WidgetType::Unspecified => "WIDGET_TYPE_UNSPECIFIED",
                WidgetType::DatePicker => "DATE_PICKER",
                WidgetType::StyledButtons => "STYLED_BUTTONS",
                WidgetType::PersistentForms => "PERSISTENT_FORMS",
                WidgetType::FixedFooter => "FIXED_FOOTER",
                WidgetType::UpdateSubjectAndRecipients => "UPDATE_SUBJECT_AND_RECIPIENTS",
                WidgetType::GridWidget => "GRID_WIDGET",
                WidgetType::AddonComposeUiAction => "ADDON_COMPOSE_UI_ACTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WIDGET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DATE_PICKER" => Some(Self::DatePicker),
                "STYLED_BUTTONS" => Some(Self::StyledButtons),
                "PERSISTENT_FORMS" => Some(Self::PersistentForms),
                "FIXED_FOOTER" => Some(Self::FixedFooter),
                "UPDATE_SUBJECT_AND_RECIPIENTS" => Some(Self::UpdateSubjectAndRecipients),
                "GRID_WIDGET" => Some(Self::GridWidget),
                "ADDON_COMPOSE_UI_ACTION" => Some(Self::AddonComposeUiAction),
                _ => None,
            }
        }
    }
}
/// Common format for declaring a  menu item, or button, that appears within a
/// host app.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuItemExtensionPoint {
    /// Required. The endpoint to execute when this extension point is
    /// activated.
    #[prost(string, tag = "1")]
    pub run_function: ::prost::alloc::string::String,
    /// Required. User-visible text describing the action taken by activating this
    /// extension point. For example, "Insert invoice".
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// The URL for the logo image shown in the add-on toolbar.
    ///
    /// If not set, defaults to the add-on's primary logo URL.
    #[prost(string, tag = "3")]
    pub logo_url: ::prost::alloc::string::String,
}
/// Common format for declaring an add-on's home-page view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomepageExtensionPoint {
    /// Required. The endpoint to execute when this extension point is
    /// activated.
    #[prost(string, tag = "1")]
    pub run_function: ::prost::alloc::string::String,
    /// Optional. If set to `false`, disable the home-page view in this context.
    ///
    /// Defaults to `true` if unset.
    ///
    /// If an add-ons custom home-page view is disabled, an autogenerated overview
    /// card will be provided for users instead.
    #[prost(message, optional, tag = "2")]
    pub enabled: ::core::option::Option<bool>,
}
/// Format for declaring a universal action menu item extension point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversalActionExtensionPoint {
    /// Required. User-visible text describing the action taken by activating this
    /// extension point, for example, "Add a new contact".
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// Required. The action type supported on a universal action menu item. It
    /// could be either a link to open or an endpoint to execute.
    #[prost(oneof = "universal_action_extension_point::ActionType", tags = "2, 3")]
    pub action_type: ::core::option::Option<
        universal_action_extension_point::ActionType,
    >,
}
/// Nested message and enum types in `UniversalActionExtensionPoint`.
pub mod universal_action_extension_point {
    /// Required. The action type supported on a universal action menu item. It
    /// could be either a link to open or an endpoint to execute.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActionType {
        /// URL to be opened by the UniversalAction.
        #[prost(string, tag = "2")]
        OpenLink(::prost::alloc::string::String),
        /// Endpoint to be run by the UniversalAction.
        #[prost(string, tag = "3")]
        RunFunction(::prost::alloc::string::String),
    }
}
/// Add-on configuration that is shared across all add-on host applications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonAddOnManifest {
    /// Required. The display name of the add-on.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The URL for the logo image shown in the add-on toolbar.
    #[prost(string, tag = "2")]
    pub logo_url: ::prost::alloc::string::String,
    /// Common layout properties for the add-on cards.
    #[prost(message, optional, tag = "3")]
    pub layout_properties: ::core::option::Option<LayoutProperties>,
    /// The widgets used in the add-on. If this field is not specified,
    /// it indicates that default set is used.
    #[prost(message, optional, tag = "4")]
    pub add_on_widget_set: ::core::option::Option<AddOnWidgetSet>,
    /// Whether to pass locale information from host app.
    #[prost(bool, tag = "5")]
    pub use_locale_from_app: bool,
    /// Defines an endpoint that will be executed in any context, in
    /// any host. Any cards generated by this function will always be available to
    /// the user, but may be eclipsed by contextual content when this add-on
    /// declares more targeted triggers.
    #[prost(message, optional, tag = "6")]
    pub homepage_trigger: ::core::option::Option<HomepageExtensionPoint>,
    /// Defines a list of extension points in the universal action menu which
    /// serves as a setting menu for the add-on. The extension point can be
    /// link URL to open or an endpoint to execute as a form
    /// submission.
    #[prost(message, repeated, tag = "7")]
    pub universal_actions: ::prost::alloc::vec::Vec<UniversalActionExtensionPoint>,
    /// An OpenLink action
    /// can only use a URL with an HTTPS, MAILTO or TEL scheme.  For HTTPS links,
    /// the URL must also
    /// [match](/gmail/add-ons/concepts/manifests#whitelisting_urls) one of the
    /// prefixes specified in this whitelist. If the prefix omits the scheme, HTTPS
    /// is assumed.  Notice that HTTP links are automatically rewritten to HTTPS
    /// links.
    #[prost(message, optional, tag = "8")]
    pub open_link_url_prefixes: ::core::option::Option<::prost_types::ListValue>,
}
/// Card layout properties shared across all add-on host applications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayoutProperties {
    /// The primary color of the add-on. It sets the color of toolbar. If no
    /// primary color is set explicitly, the default value provided by the
    /// framework is used.
    #[prost(string, tag = "1")]
    pub primary_color: ::prost::alloc::string::String,
    /// The secondary color of the add-on. It sets the color of buttons.
    /// If primary color is set but no secondary color is set, the
    /// secondary color is the same as the primary color. If neither primary
    /// color nor secondary color is set, the default value provided by the
    /// framework is used.
    #[prost(string, tag = "2")]
    pub secondary_color: ::prost::alloc::string::String,
}
/// Options for sending requests to add-on HTTP endpoints
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpOptions {
    /// Configuration for the token sent in the HTTP Authorization header
    #[prost(enumeration = "HttpAuthorizationHeader", tag = "1")]
    pub authorization_header: i32,
}
/// Authorization header sent in add-on HTTP requests
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpAuthorizationHeader {
    /// Default value, equivalent to `SYSTEM_ID_TOKEN`
    Unspecified = 0,
    /// Send an ID token for the project-specific Google Workspace Add-ons system
    /// service account (default)
    SystemIdToken = 1,
    /// Send an ID token for the end user
    UserIdToken = 2,
    /// Do not send an Authentication header
    None = 3,
}
impl HttpAuthorizationHeader {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HttpAuthorizationHeader::Unspecified => {
                "HTTP_AUTHORIZATION_HEADER_UNSPECIFIED"
            }
            HttpAuthorizationHeader::SystemIdToken => "SYSTEM_ID_TOKEN",
            HttpAuthorizationHeader::UserIdToken => "USER_ID_TOKEN",
            HttpAuthorizationHeader::None => "NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP_AUTHORIZATION_HEADER_UNSPECIFIED" => Some(Self::Unspecified),
            "SYSTEM_ID_TOKEN" => Some(Self::SystemIdToken),
            "USER_ID_TOKEN" => Some(Self::UserIdToken),
            "NONE" => Some(Self::None),
            _ => None,
        }
    }
}
