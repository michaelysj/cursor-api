/// aiserver.v1.AvailableModelsRequest
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AvailableModelsRequest {
    #[prost(bool, tag = "1")]
    pub is_nightly: bool,
    #[prost(bool, tag = "2")]
    pub include_long_context_models: bool,
}
/// aiserver.v1.AvailableModelsResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableModelsResponse {
    #[prost(message, repeated, tag = "2")]
    pub models: ::prost::alloc::vec::Vec<available_models_response::AvailableModel>,
    #[prost(string, repeated, tag = "1")]
    pub model_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AvailableModelsResponse`.
pub mod available_models_response {
    /// aiserver.v1.AvailableModelsResponse.AvailableModel
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AvailableModel {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub default_on: bool,
        #[prost(bool, optional, tag = "3")]
        pub is_long_context_only: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub is_chat_only: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub supports_agent: ::core::option::Option<bool>,
    }
}
/// aiserver.v1.ErrorDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    #[prost(enumeration = "error_details::Error", tag = "1")]
    pub error: i32,
    #[prost(message, optional, tag = "2")]
    pub details: ::core::option::Option<CustomErrorDetails>,
    #[prost(bool, optional, tag = "3")]
    pub is_expected: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ErrorDetails`.
pub mod error_details {
    /// aiserver.v1.ErrorDetails.Error
    #[allow(clippy::enum_variant_names)]
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
    pub enum Error {
        Unspecified = 0,
        BadApiKey = 1,
        BadUserApiKey = 42,
        NotLoggedIn = 2,
        InvalidAuthId = 3,
        NotHighEnoughPermissions = 4,
        AgentRequiresLogin = 18,
        BadModelName = 5,
        NotFound = 39,
        Deprecated = 40,
        UserNotFound = 6,
        FreeUserRateLimitExceeded = 7,
        ProUserRateLimitExceeded = 8,
        FreeUserUsageLimit = 9,
        ProUserUsageLimit = 10,
        ResourceExhausted = 41,
        AuthTokenNotFound = 11,
        AuthTokenExpired = 12,
        Openai = 13,
        OpenaiRateLimitExceeded = 14,
        OpenaiAccountLimitExceeded = 15,
        TaskUuidNotFound = 16,
        TaskNoPermissions = 17,
        AgentEngineNotFound = 19,
        MaxTokens = 20,
        ProUserOnly = 23,
        ApiKeyNotSupported = 24,
        UserAbortedRequest = 21,
        Timeout = 25,
        GenericRateLimitExceeded = 22,
        SlashEditFileTooLong = 26,
        FileUnsupported = 27,
        Gpt4VisionPreviewRateLimit = 28,
        CustomMessage = 29,
        OutdatedClient = 30,
        ClaudeImageTooLarge = 31,
        GitgraphNotFound = 32,
        FileNotFound = 33,
        ApiKeyRateLimit = 34,
        Debounced = 35,
        BadRequest = 36,
        RepositoryServiceRepositoryIsNotInitialized = 37,
        Unauthorized = 38,
        ConversationTooLong = 43,
    }
    impl Error {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "ERROR_UNSPECIFIED",
                Self::BadApiKey => "ERROR_BAD_API_KEY",
                Self::BadUserApiKey => "ERROR_BAD_USER_API_KEY",
                Self::NotLoggedIn => "ERROR_NOT_LOGGED_IN",
                Self::InvalidAuthId => "ERROR_INVALID_AUTH_ID",
                Self::NotHighEnoughPermissions => "ERROR_NOT_HIGH_ENOUGH_PERMISSIONS",
                Self::AgentRequiresLogin => "ERROR_AGENT_REQUIRES_LOGIN",
                Self::BadModelName => "ERROR_BAD_MODEL_NAME",
                Self::NotFound => "ERROR_NOT_FOUND",
                Self::Deprecated => "ERROR_DEPRECATED",
                Self::UserNotFound => "ERROR_USER_NOT_FOUND",
                Self::FreeUserRateLimitExceeded => "ERROR_FREE_USER_RATE_LIMIT_EXCEEDED",
                Self::ProUserRateLimitExceeded => "ERROR_PRO_USER_RATE_LIMIT_EXCEEDED",
                Self::FreeUserUsageLimit => "ERROR_FREE_USER_USAGE_LIMIT",
                Self::ProUserUsageLimit => "ERROR_PRO_USER_USAGE_LIMIT",
                Self::ResourceExhausted => "ERROR_RESOURCE_EXHAUSTED",
                Self::AuthTokenNotFound => "ERROR_AUTH_TOKEN_NOT_FOUND",
                Self::AuthTokenExpired => "ERROR_AUTH_TOKEN_EXPIRED",
                Self::Openai => "ERROR_OPENAI",
                Self::OpenaiRateLimitExceeded => "ERROR_OPENAI_RATE_LIMIT_EXCEEDED",
                Self::OpenaiAccountLimitExceeded => "ERROR_OPENAI_ACCOUNT_LIMIT_EXCEEDED",
                Self::TaskUuidNotFound => "ERROR_TASK_UUID_NOT_FOUND",
                Self::TaskNoPermissions => "ERROR_TASK_NO_PERMISSIONS",
                Self::AgentEngineNotFound => "ERROR_AGENT_ENGINE_NOT_FOUND",
                Self::MaxTokens => "ERROR_MAX_TOKENS",
                Self::ProUserOnly => "ERROR_PRO_USER_ONLY",
                Self::ApiKeyNotSupported => "ERROR_API_KEY_NOT_SUPPORTED",
                Self::UserAbortedRequest => "ERROR_USER_ABORTED_REQUEST",
                Self::Timeout => "ERROR_TIMEOUT",
                Self::GenericRateLimitExceeded => "ERROR_GENERIC_RATE_LIMIT_EXCEEDED",
                Self::SlashEditFileTooLong => "ERROR_SLASH_EDIT_FILE_TOO_LONG",
                Self::FileUnsupported => "ERROR_FILE_UNSUPPORTED",
                Self::Gpt4VisionPreviewRateLimit => {
                    "ERROR_GPT_4_VISION_PREVIEW_RATE_LIMIT"
                }
                Self::CustomMessage => "ERROR_CUSTOM_MESSAGE",
                Self::OutdatedClient => "ERROR_OUTDATED_CLIENT",
                Self::ClaudeImageTooLarge => "ERROR_CLAUDE_IMAGE_TOO_LARGE",
                Self::GitgraphNotFound => "ERROR_GITGRAPH_NOT_FOUND",
                Self::FileNotFound => "ERROR_FILE_NOT_FOUND",
                Self::ApiKeyRateLimit => "ERROR_API_KEY_RATE_LIMIT",
                Self::Debounced => "ERROR_DEBOUNCED",
                Self::BadRequest => "ERROR_BAD_REQUEST",
                Self::RepositoryServiceRepositoryIsNotInitialized => {
                    "ERROR_REPOSITORY_SERVICE_REPOSITORY_IS_NOT_INITIALIZED"
                }
                Self::Unauthorized => "ERROR_UNAUTHORIZED",
                Self::ConversationTooLong => "ERROR_CONVERSATION_TOO_LONG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_UNSPECIFIED" => Some(Self::Unspecified),
                "ERROR_BAD_API_KEY" => Some(Self::BadApiKey),
                "ERROR_BAD_USER_API_KEY" => Some(Self::BadUserApiKey),
                "ERROR_NOT_LOGGED_IN" => Some(Self::NotLoggedIn),
                "ERROR_INVALID_AUTH_ID" => Some(Self::InvalidAuthId),
                "ERROR_NOT_HIGH_ENOUGH_PERMISSIONS" => {
                    Some(Self::NotHighEnoughPermissions)
                }
                "ERROR_AGENT_REQUIRES_LOGIN" => Some(Self::AgentRequiresLogin),
                "ERROR_BAD_MODEL_NAME" => Some(Self::BadModelName),
                "ERROR_NOT_FOUND" => Some(Self::NotFound),
                "ERROR_DEPRECATED" => Some(Self::Deprecated),
                "ERROR_USER_NOT_FOUND" => Some(Self::UserNotFound),
                "ERROR_FREE_USER_RATE_LIMIT_EXCEEDED" => {
                    Some(Self::FreeUserRateLimitExceeded)
                }
                "ERROR_PRO_USER_RATE_LIMIT_EXCEEDED" => {
                    Some(Self::ProUserRateLimitExceeded)
                }
                "ERROR_FREE_USER_USAGE_LIMIT" => Some(Self::FreeUserUsageLimit),
                "ERROR_PRO_USER_USAGE_LIMIT" => Some(Self::ProUserUsageLimit),
                "ERROR_RESOURCE_EXHAUSTED" => Some(Self::ResourceExhausted),
                "ERROR_AUTH_TOKEN_NOT_FOUND" => Some(Self::AuthTokenNotFound),
                "ERROR_AUTH_TOKEN_EXPIRED" => Some(Self::AuthTokenExpired),
                "ERROR_OPENAI" => Some(Self::Openai),
                "ERROR_OPENAI_RATE_LIMIT_EXCEEDED" => Some(Self::OpenaiRateLimitExceeded),
                "ERROR_OPENAI_ACCOUNT_LIMIT_EXCEEDED" => {
                    Some(Self::OpenaiAccountLimitExceeded)
                }
                "ERROR_TASK_UUID_NOT_FOUND" => Some(Self::TaskUuidNotFound),
                "ERROR_TASK_NO_PERMISSIONS" => Some(Self::TaskNoPermissions),
                "ERROR_AGENT_ENGINE_NOT_FOUND" => Some(Self::AgentEngineNotFound),
                "ERROR_MAX_TOKENS" => Some(Self::MaxTokens),
                "ERROR_PRO_USER_ONLY" => Some(Self::ProUserOnly),
                "ERROR_API_KEY_NOT_SUPPORTED" => Some(Self::ApiKeyNotSupported),
                "ERROR_USER_ABORTED_REQUEST" => Some(Self::UserAbortedRequest),
                "ERROR_TIMEOUT" => Some(Self::Timeout),
                "ERROR_GENERIC_RATE_LIMIT_EXCEEDED" => {
                    Some(Self::GenericRateLimitExceeded)
                }
                "ERROR_SLASH_EDIT_FILE_TOO_LONG" => Some(Self::SlashEditFileTooLong),
                "ERROR_FILE_UNSUPPORTED" => Some(Self::FileUnsupported),
                "ERROR_GPT_4_VISION_PREVIEW_RATE_LIMIT" => {
                    Some(Self::Gpt4VisionPreviewRateLimit)
                }
                "ERROR_CUSTOM_MESSAGE" => Some(Self::CustomMessage),
                "ERROR_OUTDATED_CLIENT" => Some(Self::OutdatedClient),
                "ERROR_CLAUDE_IMAGE_TOO_LARGE" => Some(Self::ClaudeImageTooLarge),
                "ERROR_GITGRAPH_NOT_FOUND" => Some(Self::GitgraphNotFound),
                "ERROR_FILE_NOT_FOUND" => Some(Self::FileNotFound),
                "ERROR_API_KEY_RATE_LIMIT" => Some(Self::ApiKeyRateLimit),
                "ERROR_DEBOUNCED" => Some(Self::Debounced),
                "ERROR_BAD_REQUEST" => Some(Self::BadRequest),
                "ERROR_REPOSITORY_SERVICE_REPOSITORY_IS_NOT_INITIALIZED" => {
                    Some(Self::RepositoryServiceRepositoryIsNotInitialized)
                }
                "ERROR_UNAUTHORIZED" => Some(Self::Unauthorized),
                "ERROR_CONVERSATION_TOO_LONG" => Some(Self::ConversationTooLong),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.CustomErrorDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomErrorDetails {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "3")]
    pub allow_command_links_potentially_unsafe_please_only_use_for_handwritten_trusted_markdown: ::core::option::Option<
        bool,
    >,
    #[prost(bool, optional, tag = "4")]
    pub is_retryable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub show_request_id: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub should_show_immediate_error: ::core::option::Option<bool>,
}
/// aiserver.v1.GetChatRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChatRequest {
    #[prost(message, optional, tag = "1")]
    pub current_file: ::core::option::Option<CurrentFileInfo>,
    #[prost(message, repeated, tag = "2")]
    pub conversation: ::prost::alloc::vec::Vec<ConversationMessage>,
    #[prost(message, repeated, tag = "3")]
    pub repositories: ::prost::alloc::vec::Vec<RepositoryInfo>,
    #[prost(message, optional, tag = "4")]
    pub explicit_context: ::core::option::Option<ExplicitContext>,
    #[prost(string, optional, tag = "5")]
    pub workspace_root_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub code_blocks: ::prost::alloc::vec::Vec<CodeBlock>,
    #[prost(message, optional, tag = "7")]
    pub model_details: ::core::option::Option<ModelDetails>,
    #[prost(string, repeated, tag = "8")]
    pub documentation_identifiers: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "9")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub linter_errors: ::core::option::Option<LinterErrors>,
    #[prost(string, optional, tag = "11")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "12")]
    pub summary_up_until_index: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "13")]
    pub allow_long_file_scan: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub is_bash: ::core::option::Option<bool>,
    #[prost(string, tag = "15")]
    pub conversation_id: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "16")]
    pub can_handle_filenames_after_language_ids: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "17")]
    pub use_web: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "18")]
    pub quotes: ::prost::alloc::vec::Vec<ChatQuote>,
    #[prost(message, optional, tag = "19")]
    pub debug_info: ::core::option::Option<DebugInfo>,
    #[prost(string, optional, tag = "20")]
    pub workspace_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "21")]
    pub external_links: ::prost::alloc::vec::Vec<ChatExternalLink>,
    #[prost(message, repeated, tag = "23")]
    pub commit_notes: ::prost::alloc::vec::Vec<CommitNote>,
    #[prost(bool, optional, tag = "22")]
    pub long_context_mode: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "24")]
    pub is_eval: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "26")]
    pub desired_max_tokens: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "25")]
    pub context_ast: ::core::option::Option<ContextAst>,
    #[prost(bool, optional, tag = "27")]
    pub is_composer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "28")]
    pub runnable_code_blocks: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "29")]
    pub should_cache: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "30")]
    pub allow_model_fallbacks: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "31")]
    pub number_of_times_shown_fallback_model_warning: ::core::option::Option<i32>,
}
/// aiserver.v1.RepositoryInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryInfo {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub remote_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub remote_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub repo_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub repo_owner: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_tracked: bool,
    #[prost(bool, tag = "7")]
    pub is_local: bool,
    #[prost(int32, optional, tag = "8")]
    pub num_files: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "9")]
    pub orthogonal_transform_seed: ::core::option::Option<f64>,
    #[prost(enumeration = "EmbeddingModel", optional, tag = "10")]
    pub preferred_embedding_model: ::core::option::Option<i32>,
}
/// aiserver.v1.ModelDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelDetails {
    #[prost(string, optional, tag = "1")]
    pub model_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub api_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub enable_ghost_mode: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "4")]
    pub azure_state: ::core::option::Option<AzureState>,
    #[prost(bool, optional, tag = "5")]
    pub enable_slow_pool: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub openai_api_base_url: ::core::option::Option<::prost::alloc::string::String>,
}
/// aiserver.v1.AzureState
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureState {
    #[prost(string, tag = "1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub deployment: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub use_azure: bool,
}
/// aiserver.v1.LinterErrors
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinterErrors {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<LinterError>,
    #[prost(string, tag = "3")]
    pub file_contents: ::prost::alloc::string::String,
}
/// aiserver.v1.LinterError
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinterError {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<CursorRange>,
    #[prost(string, optional, tag = "3")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub related_information: ::prost::alloc::vec::Vec<diagnostic::RelatedInformation>,
    #[prost(enumeration = "diagnostic::DiagnosticSeverity", optional, tag = "5")]
    pub severity: ::core::option::Option<i32>,
}
/// aiserver.v1.CursorRange
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CursorRange {
    #[prost(message, optional, tag = "1")]
    pub start_position: ::core::option::Option<CursorPosition>,
    #[prost(message, optional, tag = "2")]
    pub end_position: ::core::option::Option<CursorPosition>,
}
/// aiserver.v1.CursorPosition
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CursorPosition {
    #[prost(int32, tag = "1")]
    pub line: i32,
    #[prost(int32, tag = "2")]
    pub column: i32,
}
/// aiserver.v1.ExplicitContext
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContext {
    #[prost(string, tag = "1")]
    pub context: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub repo_context: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<CursorRule>,
}
/// aiserver.v1.CursorRule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorRule {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_from_glob: ::core::option::Option<bool>,
}
/// aiserver.v1.DebugInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInfo {
    #[prost(message, optional, tag = "1")]
    pub breakpoint: ::core::option::Option<debug_info::Breakpoint>,
    #[prost(message, repeated, tag = "2")]
    pub call_stack: ::prost::alloc::vec::Vec<debug_info::CallStackFrame>,
    #[prost(message, repeated, tag = "3")]
    pub history: ::prost::alloc::vec::Vec<CodeBlock>,
}
/// Nested message and enum types in `DebugInfo`.
pub mod debug_info {
    /// aiserver.v1.DebugInfo.Variable
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Variable {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "3")]
        pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.DebugInfo.Scope
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scope {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub variables: ::prost::alloc::vec::Vec<Variable>,
    }
    /// aiserver.v1.DebugInfo.CallStackFrame
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallStackFrame {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub line_number: i32,
        #[prost(string, tag = "3")]
        pub function_name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "4")]
        pub scopes: ::prost::alloc::vec::Vec<Scope>,
    }
    /// aiserver.v1.DebugInfo.Breakpoint
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Breakpoint {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub line_number: i32,
        #[prost(string, repeated, tag = "3")]
        pub lines_before_breakpoint: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, repeated, tag = "4")]
        pub lines_after_breakpoint: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, optional, tag = "5")]
        pub exception_info: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// aiserver.v1.CodeBlock
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeBlock {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub file_contents: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub file_contents_length: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<CursorRange>,
    #[prost(string, tag = "4")]
    pub contents: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub signatures: ::core::option::Option<code_block::Signatures>,
    #[prost(string, optional, tag = "6")]
    pub override_contents: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub original_contents: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub detailed_lines: ::prost::alloc::vec::Vec<DetailedLine>,
}
/// Nested message and enum types in `CodeBlock`.
pub mod code_block {
    /// aiserver.v1.CodeBlock.Signatures
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Signatures {
        #[prost(message, repeated, tag = "1")]
        pub ranges: ::prost::alloc::vec::Vec<super::CursorRange>,
    }
}
/// aiserver.v1.DetailedLine
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailedLine {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub line_number: f32,
    #[prost(bool, tag = "3")]
    pub is_signature: bool,
}
/// aiserver.v1.CurrentFileInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentFileInfo {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contents: ::prost::alloc::string::String,
    #[prost(bool, tag = "18")]
    pub rely_on_filesync: bool,
    #[prost(string, optional, tag = "17")]
    pub sha_256_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "16")]
    pub cells: ::prost::alloc::vec::Vec<current_file_info::NotebookCell>,
    #[prost(message, repeated, tag = "10")]
    pub top_chunks: ::prost::alloc::vec::Vec<Bm25Chunk>,
    #[prost(int32, tag = "9")]
    pub contents_start_at_line: i32,
    #[prost(message, optional, tag = "3")]
    pub cursor_position: ::core::option::Option<CursorPosition>,
    #[prost(message, repeated, tag = "4")]
    pub dataframes: ::prost::alloc::vec::Vec<DataframeInfo>,
    #[prost(int32, tag = "8")]
    pub total_number_of_lines: i32,
    #[prost(string, tag = "5")]
    pub language_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub selection: ::core::option::Option<CursorRange>,
    #[prost(int32, optional, tag = "11")]
    pub alternative_version_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "7")]
    pub diagnostics: ::prost::alloc::vec::Vec<Diagnostic>,
    #[prost(int32, optional, tag = "14")]
    pub file_version: ::core::option::Option<i32>,
    #[prost(int32, repeated, tag = "15")]
    pub cell_start_lines: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "19")]
    pub workspace_root_path: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CurrentFileInfo`.
pub mod current_file_info {
    /// aiserver.v1.CurrentFileInfo.NotebookCell
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct NotebookCell {}
}
/// aiserver.v1.Diagnostic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<CursorRange>,
    #[prost(enumeration = "diagnostic::DiagnosticSeverity", tag = "3")]
    pub severity: i32,
    #[prost(message, repeated, tag = "4")]
    pub related_information: ::prost::alloc::vec::Vec<diagnostic::RelatedInformation>,
}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
    /// aiserver.v1.Diagnostic.RelatedInformation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RelatedInformation {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub range: ::core::option::Option<super::CursorRange>,
    }
    /// aiserver.v1.Diagnostic.DiagnosticSeverity
    #[allow(clippy::enum_variant_names)]
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
    pub enum DiagnosticSeverity {
        Unspecified = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
        Hint = 4,
    }
    impl DiagnosticSeverity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DIAGNOSTIC_SEVERITY_UNSPECIFIED",
                Self::Error => "DIAGNOSTIC_SEVERITY_ERROR",
                Self::Warning => "DIAGNOSTIC_SEVERITY_WARNING",
                Self::Information => "DIAGNOSTIC_SEVERITY_INFORMATION",
                Self::Hint => "DIAGNOSTIC_SEVERITY_HINT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIAGNOSTIC_SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "DIAGNOSTIC_SEVERITY_ERROR" => Some(Self::Error),
                "DIAGNOSTIC_SEVERITY_WARNING" => Some(Self::Warning),
                "DIAGNOSTIC_SEVERITY_INFORMATION" => Some(Self::Information),
                "DIAGNOSTIC_SEVERITY_HINT" => Some(Self::Hint),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.DataframeInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataframeInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shape: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub data_dimensionality: i32,
    #[prost(message, repeated, tag = "6")]
    pub columns: ::prost::alloc::vec::Vec<dataframe_info::Column>,
    #[prost(int32, tag = "7")]
    pub row_count: i32,
    #[prost(string, tag = "8")]
    pub index_column: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DataframeInfo`.
pub mod dataframe_info {
    /// aiserver.v1.DataframeInfo.Column
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Column {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.BM25Chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bm25Chunk {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<SimplestRange>,
    #[prost(int32, tag = "3")]
    pub score: i32,
    #[prost(string, tag = "4")]
    pub relative_path: ::prost::alloc::string::String,
}
/// aiserver.v1.SimplestRange
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SimplestRange {
    #[prost(int32, tag = "1")]
    pub start_line: i32,
    #[prost(int32, tag = "2")]
    pub end_line_inclusive: i32,
}
/// aiserver.v1.ConversationMessage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationMessage {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(enumeration = "conversation_message::MessageType", tag = "2")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "3")]
    pub attached_code_chunks: ::prost::alloc::vec::Vec<conversation_message::CodeChunk>,
    #[prost(message, repeated, tag = "4")]
    pub codebase_context_chunks: ::prost::alloc::vec::Vec<CodeBlock>,
    #[prost(message, repeated, tag = "5")]
    pub commits: ::prost::alloc::vec::Vec<Commit>,
    #[prost(message, repeated, tag = "6")]
    pub pull_requests: ::prost::alloc::vec::Vec<PullRequest>,
    #[prost(message, repeated, tag = "7")]
    pub git_diffs: ::prost::alloc::vec::Vec<GitDiff>,
    #[prost(message, repeated, tag = "8")]
    pub assistant_suggested_diffs: ::prost::alloc::vec::Vec<SimpleFileDiff>,
    #[prost(message, repeated, tag = "9")]
    pub interpreter_results: ::prost::alloc::vec::Vec<InterpreterResult>,
    #[prost(message, repeated, tag = "10")]
    pub images: ::prost::alloc::vec::Vec<ImageProto>,
    #[prost(string, repeated, tag = "11")]
    pub attached_folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "12")]
    pub approximate_lint_errors: ::prost::alloc::vec::Vec<
        conversation_message::ApproximateLintError,
    >,
    #[prost(string, tag = "13")]
    pub bubble_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "32")]
    pub server_bubble_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "14")]
    pub attached_folders_new: ::prost::alloc::vec::Vec<FolderInfo>,
    #[prost(message, repeated, tag = "15")]
    pub lints: ::prost::alloc::vec::Vec<conversation_message::Lints>,
    #[prost(message, repeated, tag = "16")]
    pub user_responses_to_suggested_code_blocks: ::prost::alloc::vec::Vec<
        UserResponseToSuggestedCodeBlock,
    >,
    #[prost(string, repeated, tag = "17")]
    pub relevant_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "18")]
    pub tool_results: ::prost::alloc::vec::Vec<conversation_message::ToolResult>,
    #[prost(message, repeated, tag = "19")]
    pub notepads: ::prost::alloc::vec::Vec<conversation_message::NotepadContext>,
    #[prost(bool, optional, tag = "20")]
    pub is_capability_iteration: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "21")]
    pub capabilities: ::prost::alloc::vec::Vec<ComposerCapabilityRequest>,
    #[prost(message, repeated, tag = "22")]
    pub edit_trail_contexts: ::prost::alloc::vec::Vec<
        conversation_message::EditTrailContext,
    >,
    #[prost(message, repeated, tag = "23")]
    pub suggested_code_blocks: ::prost::alloc::vec::Vec<SuggestedCodeBlock>,
    #[prost(message, repeated, tag = "24")]
    pub diffs_for_compressing_files: ::prost::alloc::vec::Vec<RedDiff>,
    #[prost(message, repeated, tag = "25")]
    pub multi_file_linter_errors: ::prost::alloc::vec::Vec<
        LinterErrorsWithoutFileContents,
    >,
    #[prost(message, repeated, tag = "26")]
    pub diff_histories: ::prost::alloc::vec::Vec<DiffHistoryData>,
    #[prost(message, repeated, tag = "27")]
    pub recently_viewed_files: ::prost::alloc::vec::Vec<conversation_message::CodeChunk>,
    #[prost(message, repeated, tag = "28")]
    pub recent_locations_history: ::prost::alloc::vec::Vec<
        conversation_message::RecentLocation,
    >,
    #[prost(bool, tag = "29")]
    pub is_agentic: bool,
    #[prost(message, repeated, tag = "30")]
    pub file_diff_trajectories: ::prost::alloc::vec::Vec<ComposerFileDiffHistory>,
    #[prost(message, optional, tag = "31")]
    pub conversation_summary: ::core::option::Option<ConversationSummary>,
    #[prost(bool, tag = "33")]
    pub existed_subsequent_terminal_command: bool,
    #[prost(bool, tag = "34")]
    pub existed_previous_terminal_command: bool,
    #[prost(message, repeated, tag = "35")]
    pub docs_references: ::prost::alloc::vec::Vec<DocsReference>,
    #[prost(message, repeated, tag = "36")]
    pub web_references: ::prost::alloc::vec::Vec<WebReference>,
    #[prost(message, optional, tag = "37")]
    pub git_context: ::core::option::Option<ViewableGitContext>,
    #[prost(message, repeated, tag = "38")]
    pub attached_folders_list_dir_results: ::prost::alloc::vec::Vec<ListDirResult>,
    #[prost(message, optional, tag = "39")]
    pub cached_conversation_summary: ::core::option::Option<ConversationSummary>,
    #[prost(message, repeated, tag = "40")]
    pub human_changes: ::prost::alloc::vec::Vec<conversation_message::HumanChange>,
    #[prost(bool, tag = "41")]
    pub attached_human_changes: bool,
    #[prost(message, repeated, tag = "42")]
    pub summarized_composers: ::prost::alloc::vec::Vec<
        conversation_message::ComposerContext,
    >,
    #[prost(message, repeated, tag = "43")]
    pub cursor_rules: ::prost::alloc::vec::Vec<CursorRule>,
}
/// Nested message and enum types in `ConversationMessage`.
pub mod conversation_message {
    /// aiserver.v1.ConversationMessage.CodeChunk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodeChunk {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub start_line_number: i32,
        #[prost(string, repeated, tag = "3")]
        pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(enumeration = "code_chunk::SummarizationStrategy", optional, tag = "4")]
        pub summarization_strategy: ::core::option::Option<i32>,
        #[prost(string, tag = "5")]
        pub language_identifier: ::prost::alloc::string::String,
        #[prost(enumeration = "code_chunk::Intent", optional, tag = "6")]
        pub intent: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "7")]
        pub is_final_version: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "8")]
        pub is_first_version: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "9")]
        pub contents_are_missing: ::core::option::Option<bool>,
    }
    /// Nested message and enum types in `CodeChunk`.
    pub mod code_chunk {
        /// aiserver.v1.ConversationMessage.CodeChunk.Intent
        #[allow(clippy::enum_variant_names)]
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
        pub enum Intent {
            Unspecified = 0,
            ComposerFile = 1,
            CompressedComposerFile = 2,
            RecentlyViewedFile = 3,
            Outline = 4,
            MentionedFile = 5,
            CodeSelection = 6,
        }
        impl Intent {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "INTENT_UNSPECIFIED",
                    Self::ComposerFile => "INTENT_COMPOSER_FILE",
                    Self::CompressedComposerFile => "INTENT_COMPRESSED_COMPOSER_FILE",
                    Self::RecentlyViewedFile => "INTENT_RECENTLY_VIEWED_FILE",
                    Self::Outline => "INTENT_OUTLINE",
                    Self::MentionedFile => "INTENT_MENTIONED_FILE",
                    Self::CodeSelection => "INTENT_CODE_SELECTION",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "INTENT_UNSPECIFIED" => Some(Self::Unspecified),
                    "INTENT_COMPOSER_FILE" => Some(Self::ComposerFile),
                    "INTENT_COMPRESSED_COMPOSER_FILE" => {
                        Some(Self::CompressedComposerFile)
                    }
                    "INTENT_RECENTLY_VIEWED_FILE" => Some(Self::RecentlyViewedFile),
                    "INTENT_OUTLINE" => Some(Self::Outline),
                    "INTENT_MENTIONED_FILE" => Some(Self::MentionedFile),
                    "INTENT_CODE_SELECTION" => Some(Self::CodeSelection),
                    _ => None,
                }
            }
        }
        /// aiserver.v1.ConversationMessage.CodeChunk.SummarizationStrategy
        #[allow(clippy::enum_variant_names)]
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
        pub enum SummarizationStrategy {
            NoneUnspecified = 0,
            Summarized = 1,
            Embedded = 2,
        }
        impl SummarizationStrategy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::NoneUnspecified => "SUMMARIZATION_STRATEGY_NONE_UNSPECIFIED",
                    Self::Summarized => "SUMMARIZATION_STRATEGY_SUMMARIZED",
                    Self::Embedded => "SUMMARIZATION_STRATEGY_EMBEDDED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SUMMARIZATION_STRATEGY_NONE_UNSPECIFIED" => {
                        Some(Self::NoneUnspecified)
                    }
                    "SUMMARIZATION_STRATEGY_SUMMARIZED" => Some(Self::Summarized),
                    "SUMMARIZATION_STRATEGY_EMBEDDED" => Some(Self::Embedded),
                    _ => None,
                }
            }
        }
    }
    /// aiserver.v1.ConversationMessage.ToolResult
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolResult {
        #[prost(string, tag = "1")]
        pub tool_call_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub tool_name: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub tool_index: u32,
        #[prost(string, tag = "4")]
        pub args: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub raw_args: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "6")]
        pub attached_code_chunks: ::prost::alloc::vec::Vec<CodeChunk>,
        #[prost(string, optional, tag = "7")]
        pub content: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "8")]
        pub result: ::core::option::Option<super::ClientSideToolV2Result>,
        #[prost(message, optional, tag = "9")]
        pub error: ::core::option::Option<super::ToolResultError>,
        #[prost(message, repeated, tag = "10")]
        pub images: ::prost::alloc::vec::Vec<super::ImageProto>,
    }
    /// aiserver.v1.ConversationMessage.MultiRangeCodeChunk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiRangeCodeChunk {
        #[prost(message, repeated, tag = "1")]
        pub ranges: ::prost::alloc::vec::Vec<multi_range_code_chunk::RangeWithPriority>,
        #[prost(string, tag = "2")]
        pub content: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub relative_workspace_path: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `MultiRangeCodeChunk`.
    pub mod multi_range_code_chunk {
        /// aiserver.v1.ConversationMessage.MultiRangeCodeChunk.RangeWithPriority
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct RangeWithPriority {
            #[prost(message, optional, tag = "1")]
            pub range: ::core::option::Option<super::super::SimplestRange>,
            #[prost(double, tag = "2")]
            pub priority: f64,
        }
    }
    /// aiserver.v1.ConversationMessage.NotepadContext
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotepadContext {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub attached_code_chunks: ::prost::alloc::vec::Vec<CodeChunk>,
        #[prost(string, repeated, tag = "4")]
        pub attached_folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "5")]
        pub commits: ::prost::alloc::vec::Vec<super::Commit>,
        #[prost(message, repeated, tag = "6")]
        pub pull_requests: ::prost::alloc::vec::Vec<super::PullRequest>,
        #[prost(message, repeated, tag = "7")]
        pub git_diffs: ::prost::alloc::vec::Vec<super::GitDiff>,
        #[prost(message, repeated, tag = "8")]
        pub images: ::prost::alloc::vec::Vec<super::ImageProto>,
    }
    /// aiserver.v1.ConversationMessage.ComposerContext
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComposerContext {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub conversation_summary: ::core::option::Option<super::ConversationSummary>,
    }
    /// aiserver.v1.ConversationMessage.EditLocation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditLocation {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub range: ::core::option::Option<super::SimplestRange>,
        #[prost(message, optional, tag = "4")]
        pub initial_range: ::core::option::Option<super::SimplestRange>,
        #[prost(string, tag = "5")]
        pub context_lines: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub text: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "7")]
        pub text_range: ::core::option::Option<super::SimplestRange>,
    }
    /// aiserver.v1.ConversationMessage.EditTrailContext
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditTrailContext {
        #[prost(string, tag = "1")]
        pub unique_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub edit_trail_sorted: ::prost::alloc::vec::Vec<EditLocation>,
    }
    /// aiserver.v1.ConversationMessage.ApproximateLintError
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproximateLintError {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub start_line: i32,
        #[prost(int32, tag = "4")]
        pub end_line: i32,
        #[prost(int32, tag = "5")]
        pub start_column: i32,
        #[prost(int32, tag = "6")]
        pub end_column: i32,
    }
    /// aiserver.v1.ConversationMessage.Lints
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Lints {
        #[prost(message, optional, tag = "1")]
        pub lints: ::core::option::Option<super::GetLintsForChangeResponse>,
        #[prost(string, tag = "2")]
        pub chat_codeblock_model_value: ::prost::alloc::string::String,
    }
    /// aiserver.v1.ConversationMessage.RecentLocation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecentLocation {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub line_number: i32,
    }
    /// aiserver.v1.ConversationMessage.RenderedDiff
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RenderedDiff {
        #[prost(int32, tag = "1")]
        pub start_line_number: i32,
        #[prost(int32, tag = "2")]
        pub end_line_number_exclusive: i32,
        #[prost(string, repeated, tag = "3")]
        pub before_context_lines: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, repeated, tag = "4")]
        pub removed_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "5")]
        pub added_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "6")]
        pub after_context_lines: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// aiserver.v1.ConversationMessage.HumanChange
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HumanChange {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub rendered_diffs: ::prost::alloc::vec::Vec<RenderedDiff>,
    }
    /// aiserver.v1.ConversationMessage.MessageType
    #[allow(clippy::enum_variant_names)]
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
    pub enum MessageType {
        Unspecified = 0,
        Human = 1,
        Ai = 2,
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "MESSAGE_TYPE_UNSPECIFIED",
                Self::Human => "MESSAGE_TYPE_HUMAN",
                Self::Ai => "MESSAGE_TYPE_AI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MESSAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MESSAGE_TYPE_HUMAN" => Some(Self::Human),
                "MESSAGE_TYPE_AI" => Some(Self::Ai),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.WebReference
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebReference {
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chunk: ::prost::alloc::string::String,
}
/// aiserver.v1.ViewableGitContext
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableGitContext {
    #[prost(message, optional, tag = "1")]
    pub commit_data: ::core::option::Option<ViewableCommitProps>,
    #[prost(message, optional, tag = "2")]
    pub pull_request_data: ::core::option::Option<ViewablePrProps>,
    #[prost(message, repeated, tag = "3")]
    pub diff_data: ::prost::alloc::vec::Vec<ViewableDiffProps>,
}
/// aiserver.v1.ViewablePRProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewablePrProps {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub body: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub files: ::prost::alloc::vec::Vec<DiffFile>,
}
/// aiserver.v1.DiffFile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffFile {
    #[prost(string, tag = "1")]
    pub file_details: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
}
/// aiserver.v1.ViewableDiffProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableDiffProps {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<DiffFile>,
    #[prost(string, tag = "2")]
    pub diff_preface: ::prost::alloc::string::String,
}
/// aiserver.v1.ViewableCommitProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableCommitProps {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub files: ::prost::alloc::vec::Vec<DiffFile>,
}
/// aiserver.v1.UserResponseToSuggestedCodeBlock
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserResponseToSuggestedCodeBlock {
    #[prost(
        enumeration = "user_response_to_suggested_code_block::UserResponseType",
        tag = "1"
    )]
    pub user_response_type: i32,
    #[prost(string, tag = "2")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub user_modifications_to_suggested_code_blocks: ::core::option::Option<FileDiff>,
}
/// Nested message and enum types in `UserResponseToSuggestedCodeBlock`.
pub mod user_response_to_suggested_code_block {
    /// aiserver.v1.UserResponseToSuggestedCodeBlock.UserResponseType
    #[allow(clippy::enum_variant_names)]
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
    pub enum UserResponseType {
        Unspecified = 0,
        Accept = 1,
        Reject = 2,
        Modify = 3,
    }
    impl UserResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "USER_RESPONSE_TYPE_UNSPECIFIED",
                Self::Accept => "USER_RESPONSE_TYPE_ACCEPT",
                Self::Reject => "USER_RESPONSE_TYPE_REJECT",
                Self::Modify => "USER_RESPONSE_TYPE_MODIFY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_RESPONSE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "USER_RESPONSE_TYPE_ACCEPT" => Some(Self::Accept),
                "USER_RESPONSE_TYPE_REJECT" => Some(Self::Reject),
                "USER_RESPONSE_TYPE_MODIFY" => Some(Self::Modify),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.FileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDiff {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub chunks: ::prost::alloc::vec::Vec<file_diff::Chunk>,
}
/// Nested message and enum types in `FileDiff`.
pub mod file_diff {
    /// aiserver.v1.FileDiff.Chunk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Chunk {
        #[prost(string, tag = "1")]
        pub content: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(int32, tag = "3")]
        pub old_start: i32,
        #[prost(int32, tag = "4")]
        pub old_lines: i32,
        #[prost(int32, tag = "5")]
        pub new_start: i32,
        #[prost(int32, tag = "6")]
        pub new_lines: i32,
    }
}
/// aiserver.v1.ToolResultError
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultError {
    #[prost(string, tag = "1")]
    pub client_visible_error_message: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub model_visible_error_message: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub actual_error_message_only_send_from_client_to_server_never_the_other_way_around_because_that_may_be_a_security_risk: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// aiserver.v1.ToolResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResult {
    #[prost(oneof = "tool_result::ToolResult", tags = "1, 2, 3")]
    pub tool_result: ::core::option::Option<tool_result::ToolResult>,
}
/// Nested message and enum types in `ToolResult`.
pub mod tool_result {
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ToolResult {
        #[prost(message, tag = "1")]
        BuiltinToolResult(super::BuiltinToolResult),
        #[prost(message, tag = "2")]
        CustomToolResult(super::CustomToolResult),
        #[prost(message, tag = "3")]
        ErrorToolResult(super::ErrorToolResult),
    }
}
/// aiserver.v1.ErrorToolResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorToolResult {
    #[prost(string, tag = "1")]
    pub error_message: ::prost::alloc::string::String,
}
/// aiserver.v1.CustomToolResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomToolResult {
    #[prost(string, tag = "1")]
    pub tool_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub result: ::prost::alloc::string::String,
}
/// aiserver.v1.BuiltinToolResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuiltinToolResult {
    #[prost(enumeration = "BuiltinTool", tag = "1")]
    pub tool: i32,
    #[prost(
        oneof = "builtin_tool_result::Result",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24"
    )]
    pub result: ::core::option::Option<builtin_tool_result::Result>,
}
/// Nested message and enum types in `BuiltinToolResult`.
pub mod builtin_tool_result {
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        SearchResult(super::SearchResult),
        #[prost(message, tag = "3")]
        ReadChunkResult(super::ReadChunkResult),
        #[prost(message, tag = "4")]
        GotodefResult(super::GotodefResult),
        #[prost(message, tag = "5")]
        EditResult(super::EditResult),
        #[prost(message, tag = "6")]
        UndoEditResult(super::UndoEditResult),
        #[prost(message, tag = "7")]
        EndResult(super::EndResult),
        #[prost(message, tag = "8")]
        NewFileResult(super::NewFileResult),
        #[prost(message, tag = "9")]
        AddTestResult(super::AddTestResult),
        #[prost(message, tag = "10")]
        RunTestResult(super::RunTestResult),
        #[prost(message, tag = "11")]
        DeleteTestResult(super::DeleteTestResult),
        #[prost(message, tag = "12")]
        SaveFileResult(super::SaveFileResult),
        #[prost(message, tag = "13")]
        GetTestsResult(super::GetTestsResult),
        #[prost(message, tag = "14")]
        GetSymbolsResult(super::GetSymbolsResult),
        #[prost(message, tag = "15")]
        SemanticSearchResult(super::SemanticSearchResult),
        #[prost(message, tag = "16")]
        GetProjectStructureResult(super::GetProjectStructureResult),
        #[prost(message, tag = "17")]
        CreateRmFilesResult(super::CreateRmFilesResult),
        #[prost(message, tag = "18")]
        RunTerminalCommandsResult(super::RunTerminalCommandsResult),
        #[prost(message, tag = "19")]
        NewEditResult(super::NewEditResult),
        #[prost(message, tag = "20")]
        ReadWithLinterResult(super::ReadWithLinterResult),
        #[prost(message, tag = "21")]
        AddUiStepResult(super::AddUiStepResult),
        #[prost(message, tag = "22")]
        ReadSemsearchFilesResult(super::ReadSemsearchFilesResult),
        #[prost(message, tag = "23")]
        CreateFileResult(super::CreateFileResult),
        #[prost(message, tag = "24")]
        DeleteFileResult(super::DeleteFileResult),
    }
}
/// aiserver.v1.UndoEditResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoEditResult {
    #[prost(string, repeated, tag = "1")]
    pub feedback: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub context_start_line_number: i32,
    #[prost(string, repeated, tag = "3")]
    pub context_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "5")]
    pub context_total_num_lines: i32,
    #[prost(int32, tag = "6")]
    pub file_total_lines: i32,
}
/// aiserver.v1.SemanticSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticSearchResult {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<semantic_search_result::Item>,
    #[prost(map = "string, string", tag = "2")]
    pub files: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `SemanticSearchResult`.
pub mod semantic_search_result {
    /// aiserver.v1.SemanticSearchResult.Item
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(float, tag = "2")]
        pub score: f32,
        #[prost(string, tag = "3")]
        pub content: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "4")]
        pub range: ::core::option::Option<super::SimpleRange>,
        #[prost(string, optional, tag = "5")]
        pub original_content: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "6")]
        pub detailed_lines: ::prost::alloc::vec::Vec<super::DetailedLine>,
    }
}
/// aiserver.v1.SimpleRange
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SimpleRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number_inclusive: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
}
/// aiserver.v1.SearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResult {
    #[prost(message, repeated, tag = "1")]
    pub file_results: ::prost::alloc::vec::Vec<SearchToolFileSearchResult>,
    #[prost(int32, tag = "2")]
    pub num_total_matches: i32,
    #[prost(int32, tag = "3")]
    pub num_total_matched_files: i32,
    #[prost(bool, tag = "4")]
    pub num_total_may_be_incomplete: bool,
    #[prost(bool, tag = "5")]
    pub files_only: bool,
}
/// aiserver.v1.SearchToolFileSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchToolFileSearchResult {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub num_matches: i32,
    #[prost(message, repeated, tag = "3")]
    pub potentially_relevant_lines: ::prost::alloc::vec::Vec<
        search_tool_file_search_result::Line,
    >,
    #[prost(bool, tag = "4")]
    pub cropped: bool,
}
/// Nested message and enum types in `SearchToolFileSearchResult`.
pub mod search_tool_file_search_result {
    /// aiserver.v1.SearchToolFileSearchResult.Line
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Line {
        #[prost(int32, tag = "1")]
        pub line_number: i32,
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.SaveFileResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SaveFileResult {}
/// aiserver.v1.RunTestResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTestResult {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// aiserver.v1.RunTerminalCommandsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTerminalCommandsResult {
    #[prost(string, repeated, tag = "1")]
    pub outputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// aiserver.v1.ReadWithLinterResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadWithLinterResult {
    #[prost(string, tag = "1")]
    pub contents: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub diagnostics: ::prost::alloc::vec::Vec<Diagnostic>,
}
/// aiserver.v1.ReadSemsearchFilesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSemsearchFilesResult {
    #[prost(message, repeated, tag = "1")]
    pub code_results: ::prost::alloc::vec::Vec<CodeResult>,
    #[prost(message, repeated, tag = "2")]
    pub all_files: ::prost::alloc::vec::Vec<File>,
    #[prost(message, repeated, tag = "3")]
    pub missing_files: ::prost::alloc::vec::Vec<MissingFile>,
}
/// aiserver.v1.MissingFile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissingFile {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(enumeration = "missing_file::MissingReason", tag = "2")]
    pub missing_reason: i32,
    #[prost(int32, optional, tag = "3")]
    pub num_lines: ::core::option::Option<i32>,
}
/// Nested message and enum types in `MissingFile`.
pub mod missing_file {
    /// aiserver.v1.MissingFile.MissingReason
    #[allow(clippy::enum_variant_names)]
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
    pub enum MissingReason {
        Unspecified = 0,
        TooLarge = 1,
        NotFound = 2,
    }
    impl MissingReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "MISSING_REASON_UNSPECIFIED",
                Self::TooLarge => "MISSING_REASON_TOO_LARGE",
                Self::NotFound => "MISSING_REASON_NOT_FOUND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MISSING_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "MISSING_REASON_TOO_LARGE" => Some(Self::TooLarge),
                "MISSING_REASON_NOT_FOUND" => Some(Self::NotFound),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.File
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contents: ::prost::alloc::string::String,
}
/// aiserver.v1.CodeResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeResult {
    #[prost(message, optional, tag = "1")]
    pub code_block: ::core::option::Option<CodeBlock>,
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// aiserver.v1.ReadChunkResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChunkResult {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub start_line_number: i32,
    #[prost(string, repeated, tag = "3")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "4")]
    pub total_num_lines: i32,
    #[prost(bool, tag = "5")]
    pub cropped: bool,
}
/// aiserver.v1.NewFileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFileResult {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub file_total_lines: i32,
}
/// aiserver.v1.NewEditResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NewEditResult {}
/// aiserver.v1.GotodefResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GotodefResult {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GotodefToolFileSearchResult>,
}
/// aiserver.v1.GotodefToolFileSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GotodefToolFileSearchResult {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub potentially_relevant_lines: ::prost::alloc::vec::Vec<
        gotodef_tool_file_search_result::Line,
    >,
}
/// Nested message and enum types in `GotodefToolFileSearchResult`.
pub mod gotodef_tool_file_search_result {
    /// aiserver.v1.GotodefToolFileSearchResult.Line
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Line {
        #[prost(int32, tag = "1")]
        pub line_number: i32,
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.GetTestsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestsResult {
    #[prost(message, repeated, tag = "1")]
    pub tests: ::prost::alloc::vec::Vec<get_tests_result::Test>,
}
/// Nested message and enum types in `GetTestsResult`.
pub mod get_tests_result {
    /// aiserver.v1.GetTestsResult.Test
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Test {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// aiserver.v1.GetSymbolsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSymbolsResult {
    #[prost(message, repeated, tag = "1")]
    pub symbols: ::prost::alloc::vec::Vec<DocumentSymbol>,
}
/// aiserver.v1.DocumentSymbol
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSymbol {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
    #[prost(enumeration = "document_symbol::SymbolKind", tag = "3")]
    pub kind: i32,
    #[prost(string, tag = "5")]
    pub container_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub range: ::core::option::Option<document_symbol::Range>,
    #[prost(message, optional, tag = "7")]
    pub selection_range: ::core::option::Option<document_symbol::Range>,
    #[prost(message, repeated, tag = "8")]
    pub children: ::prost::alloc::vec::Vec<DocumentSymbol>,
}
/// Nested message and enum types in `DocumentSymbol`.
pub mod document_symbol {
    /// aiserver.v1.DocumentSymbol.Range
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(int32, tag = "1")]
        pub start_line_number: i32,
        #[prost(int32, tag = "2")]
        pub start_column: i32,
        #[prost(int32, tag = "3")]
        pub end_line_number: i32,
        #[prost(int32, tag = "4")]
        pub end_column: i32,
    }
    /// aiserver.v1.DocumentSymbol.SymbolKind
    #[allow(clippy::enum_variant_names)]
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
    pub enum SymbolKind {
        Unspecified = 0,
        File = 1,
        Module = 2,
        Namespace = 3,
        Package = 4,
        Class = 5,
        Method = 6,
        Property = 7,
        Field = 8,
        Constructor = 9,
        Enum = 10,
        Interface = 11,
        Function = 12,
        Variable = 13,
        Constant = 14,
        String = 15,
        Number = 16,
        Boolean = 17,
        Array = 18,
        Object = 19,
        Key = 20,
        Null = 21,
        EnumMember = 22,
        Struct = 23,
        Event = 24,
        Operator = 25,
        TypeParameter = 26,
    }
    impl SymbolKind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "SYMBOL_KIND_UNSPECIFIED",
                Self::File => "SYMBOL_KIND_FILE",
                Self::Module => "SYMBOL_KIND_MODULE",
                Self::Namespace => "SYMBOL_KIND_NAMESPACE",
                Self::Package => "SYMBOL_KIND_PACKAGE",
                Self::Class => "SYMBOL_KIND_CLASS",
                Self::Method => "SYMBOL_KIND_METHOD",
                Self::Property => "SYMBOL_KIND_PROPERTY",
                Self::Field => "SYMBOL_KIND_FIELD",
                Self::Constructor => "SYMBOL_KIND_CONSTRUCTOR",
                Self::Enum => "SYMBOL_KIND_ENUM",
                Self::Interface => "SYMBOL_KIND_INTERFACE",
                Self::Function => "SYMBOL_KIND_FUNCTION",
                Self::Variable => "SYMBOL_KIND_VARIABLE",
                Self::Constant => "SYMBOL_KIND_CONSTANT",
                Self::String => "SYMBOL_KIND_STRING",
                Self::Number => "SYMBOL_KIND_NUMBER",
                Self::Boolean => "SYMBOL_KIND_BOOLEAN",
                Self::Array => "SYMBOL_KIND_ARRAY",
                Self::Object => "SYMBOL_KIND_OBJECT",
                Self::Key => "SYMBOL_KIND_KEY",
                Self::Null => "SYMBOL_KIND_NULL",
                Self::EnumMember => "SYMBOL_KIND_ENUM_MEMBER",
                Self::Struct => "SYMBOL_KIND_STRUCT",
                Self::Event => "SYMBOL_KIND_EVENT",
                Self::Operator => "SYMBOL_KIND_OPERATOR",
                Self::TypeParameter => "SYMBOL_KIND_TYPE_PARAMETER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMBOL_KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "SYMBOL_KIND_FILE" => Some(Self::File),
                "SYMBOL_KIND_MODULE" => Some(Self::Module),
                "SYMBOL_KIND_NAMESPACE" => Some(Self::Namespace),
                "SYMBOL_KIND_PACKAGE" => Some(Self::Package),
                "SYMBOL_KIND_CLASS" => Some(Self::Class),
                "SYMBOL_KIND_METHOD" => Some(Self::Method),
                "SYMBOL_KIND_PROPERTY" => Some(Self::Property),
                "SYMBOL_KIND_FIELD" => Some(Self::Field),
                "SYMBOL_KIND_CONSTRUCTOR" => Some(Self::Constructor),
                "SYMBOL_KIND_ENUM" => Some(Self::Enum),
                "SYMBOL_KIND_INTERFACE" => Some(Self::Interface),
                "SYMBOL_KIND_FUNCTION" => Some(Self::Function),
                "SYMBOL_KIND_VARIABLE" => Some(Self::Variable),
                "SYMBOL_KIND_CONSTANT" => Some(Self::Constant),
                "SYMBOL_KIND_STRING" => Some(Self::String),
                "SYMBOL_KIND_NUMBER" => Some(Self::Number),
                "SYMBOL_KIND_BOOLEAN" => Some(Self::Boolean),
                "SYMBOL_KIND_ARRAY" => Some(Self::Array),
                "SYMBOL_KIND_OBJECT" => Some(Self::Object),
                "SYMBOL_KIND_KEY" => Some(Self::Key),
                "SYMBOL_KIND_NULL" => Some(Self::Null),
                "SYMBOL_KIND_ENUM_MEMBER" => Some(Self::EnumMember),
                "SYMBOL_KIND_STRUCT" => Some(Self::Struct),
                "SYMBOL_KIND_EVENT" => Some(Self::Event),
                "SYMBOL_KIND_OPERATOR" => Some(Self::Operator),
                "SYMBOL_KIND_TYPE_PARAMETER" => Some(Self::TypeParameter),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.GetProjectStructureResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectStructureResult {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<get_project_structure_result::File>,
    #[prost(string, tag = "2")]
    pub root_workspace_path: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GetProjectStructureResult`.
pub mod get_project_structure_result {
    /// aiserver.v1.GetProjectStructureResult.File
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub outline: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.EndResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EndResult {}
/// aiserver.v1.EditResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditResult {
    #[prost(string, repeated, tag = "1")]
    pub feedback: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "2")]
    pub context_start_line_number: i32,
    #[prost(string, repeated, tag = "3")]
    pub context_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub file: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub file_total_lines: i32,
    #[prost(message, repeated, tag = "6")]
    pub structured_feedback: ::prost::alloc::vec::Vec<edit_result::Feedback>,
}
/// Nested message and enum types in `EditResult`.
pub mod edit_result {
    /// aiserver.v1.EditResult.RelatedInformation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RelatedInformation {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub start_line_number: i32,
        #[prost(int32, tag = "3")]
        pub end_line_number: i32,
        #[prost(string, tag = "4")]
        pub relative_workspace_path: ::prost::alloc::string::String,
    }
    /// aiserver.v1.EditResult.Feedback
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Feedback {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub severity: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub start_line_number: i32,
        #[prost(int32, tag = "4")]
        pub end_line_number: i32,
        #[prost(message, repeated, tag = "5")]
        pub related_information: ::prost::alloc::vec::Vec<RelatedInformation>,
    }
}
/// aiserver.v1.DeleteTestResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DeleteTestResult {}
/// aiserver.v1.DeleteFileResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DeleteFileResult {
    #[prost(bool, tag = "1")]
    pub rejected: bool,
    #[prost(bool, tag = "2")]
    pub file_non_existent: bool,
    #[prost(bool, tag = "3")]
    pub file_deleted_successfully: bool,
}
/// aiserver.v1.CreateRmFilesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRmFilesResult {
    #[prost(string, repeated, tag = "1")]
    pub created_file_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub removed_file_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// aiserver.v1.CreateFileResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CreateFileResult {
    #[prost(bool, tag = "1")]
    pub file_created_successfully: bool,
    #[prost(bool, tag = "2")]
    pub file_already_exists: bool,
}
/// aiserver.v1.AddUiStepResult
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AddUiStepResult {}
/// aiserver.v1.AddTestResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTestResult {
    #[prost(message, repeated, tag = "1")]
    pub feedback: ::prost::alloc::vec::Vec<add_test_result::Feedback>,
}
/// Nested message and enum types in `AddTestResult`.
pub mod add_test_result {
    /// aiserver.v1.AddTestResult.RelatedInformation
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RelatedInformation {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub start_line_number: i32,
        #[prost(int32, tag = "3")]
        pub end_line_number: i32,
        #[prost(string, tag = "4")]
        pub relative_workspace_path: ::prost::alloc::string::String,
    }
    /// aiserver.v1.AddTestResult.Feedback
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Feedback {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub severity: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub start_line_number: i32,
        #[prost(int32, tag = "4")]
        pub end_line_number: i32,
        #[prost(message, repeated, tag = "5")]
        pub related_information: ::prost::alloc::vec::Vec<RelatedInformation>,
    }
}
/// aiserver.v1.SuggestedCodeBlock
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestedCodeBlock {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
}
/// aiserver.v1.SimpleFileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleFileDiff {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub chunks: ::prost::alloc::vec::Vec<simple_file_diff::Chunk>,
}
/// Nested message and enum types in `SimpleFileDiff`.
pub mod simple_file_diff {
    /// aiserver.v1.SimpleFileDiff.Chunk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Chunk {
        #[prost(string, repeated, tag = "1")]
        pub old_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "2")]
        pub new_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "3")]
        pub old_range: ::core::option::Option<super::LineRange>,
        #[prost(message, optional, tag = "4")]
        pub new_range: ::core::option::Option<super::LineRange>,
    }
}
/// aiserver.v1.LineRange
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LineRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub end_line_number_inclusive: i32,
}
/// aiserver.v1.RedDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedDiff {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub red_ranges: ::prost::alloc::vec::Vec<SimplestRange>,
    #[prost(message, repeated, tag = "3")]
    pub red_ranges_reversed: ::prost::alloc::vec::Vec<SimplestRange>,
    #[prost(string, tag = "4")]
    pub start_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub end_hash: ::prost::alloc::string::String,
}
/// aiserver.v1.PullRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequest {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub body: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub diff: ::prost::alloc::vec::Vec<FileDiff>,
}
/// aiserver.v1.ListDirResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDirResult {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<list_dir_result::File>,
    #[prost(string, tag = "2")]
    pub directory_relative_workspace_path: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListDirResult`.
pub mod list_dir_result {
    /// aiserver.v1.ListDirResult.File
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub is_directory: bool,
        #[prost(int64, optional, tag = "3")]
        pub size: ::core::option::Option<i64>,
        #[prost(message, optional, tag = "4")]
        pub last_modified: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(int32, optional, tag = "5")]
        pub num_children: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "6")]
        pub num_lines: ::core::option::Option<i32>,
    }
}
/// aiserver.v1.LinterErrorsWithoutFileContents
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinterErrorsWithoutFileContents {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<LinterError>,
}
/// aiserver.v1.InterpreterResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterpreterResult {
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub success: bool,
}
/// aiserver.v1.ImageProto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageProto {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub dimension: ::core::option::Option<image_proto::Dimension>,
}
/// Nested message and enum types in `ImageProto`.
pub mod image_proto {
    /// aiserver.v1.ImageProto.Dimension
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Dimension {
        #[prost(int32, tag = "1")]
        pub width: i32,
        #[prost(int32, tag = "2")]
        pub height: i32,
    }
}
/// aiserver.v1.GitDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitDiff {
    #[prost(message, repeated, tag = "1")]
    pub diffs: ::prost::alloc::vec::Vec<FileDiff>,
    #[prost(enumeration = "git_diff::DiffType", tag = "2")]
    pub diff_type: i32,
}
/// Nested message and enum types in `GitDiff`.
pub mod git_diff {
    /// aiserver.v1.GitDiff.DiffType
    #[allow(clippy::enum_variant_names)]
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
    pub enum DiffType {
        Unspecified = 0,
        DiffToHead = 1,
        DiffFromBranchToMain = 2,
    }
    impl DiffType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DIFF_TYPE_UNSPECIFIED",
                Self::DiffToHead => "DIFF_TYPE_DIFF_TO_HEAD",
                Self::DiffFromBranchToMain => "DIFF_TYPE_DIFF_FROM_BRANCH_TO_MAIN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIFF_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIFF_TYPE_DIFF_TO_HEAD" => Some(Self::DiffToHead),
                "DIFF_TYPE_DIFF_FROM_BRANCH_TO_MAIN" => Some(Self::DiffFromBranchToMain),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.GetLintsForChangeResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLintsForChangeResponse {
    #[prost(message, repeated, tag = "1")]
    pub lints: ::prost::alloc::vec::Vec<get_lints_for_change_response::Lint>,
}
/// Nested message and enum types in `GetLintsForChangeResponse`.
pub mod get_lints_for_change_response {
    /// aiserver.v1.GetLintsForChangeResponse.Lint
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Lint {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub severity: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(int32, tag = "4")]
        pub start_line_number_one_indexed: i32,
        #[prost(int32, tag = "5")]
        pub start_column_one_indexed: i32,
        #[prost(int32, tag = "6")]
        pub end_line_number_inclusive_one_indexed: i32,
        #[prost(int32, tag = "7")]
        pub end_column_one_indexed: i32,
        #[prost(message, repeated, tag = "9")]
        pub quick_fixes: ::prost::alloc::vec::Vec<lint::QuickFix>,
    }
    /// Nested message and enum types in `Lint`.
    pub mod lint {
        /// aiserver.v1.GetLintsForChangeResponse.Lint.QuickFix
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QuickFix {
            #[prost(string, tag = "1")]
            pub message: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub kind: ::prost::alloc::string::String,
            #[prost(bool, tag = "3")]
            pub is_preferred: bool,
            #[prost(message, repeated, tag = "4")]
            pub edits: ::prost::alloc::vec::Vec<quick_fix::Edit>,
        }
        /// Nested message and enum types in `QuickFix`.
        pub mod quick_fix {
            /// aiserver.v1.GetLintsForChangeResponse.Lint.QuickFix.Edit
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Edit {
                #[prost(string, tag = "1")]
                pub relative_workspace_path: ::prost::alloc::string::String,
                #[prost(string, tag = "2")]
                pub text: ::prost::alloc::string::String,
                #[prost(int32, tag = "3")]
                pub start_line_number_one_indexed: i32,
                #[prost(int32, tag = "4")]
                pub start_column_one_indexed: i32,
                #[prost(int32, tag = "5")]
                pub end_line_number_inclusive_one_indexed: i32,
                #[prost(int32, tag = "6")]
                pub end_column_one_indexed: i32,
            }
        }
    }
}
/// aiserver.v1.Lint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lint {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<SimpleRange>,
    #[prost(enumeration = "LintSeverity", tag = "3")]
    pub severity: i32,
}
/// aiserver.v1.Edit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edit {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<IRange>,
}
/// aiserver.v1.IRange
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct IRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
}
/// aiserver.v1.FolderInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderInfo {
    #[prost(string, tag = "1")]
    pub relative_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<FolderFileInfo>,
}
/// aiserver.v1.FolderFileInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderFileInfo {
    #[prost(string, tag = "1")]
    pub relative_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub truncated: bool,
    #[prost(float, tag = "4")]
    pub score: f32,
}
/// aiserver.v1.DocsReference
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocsReference {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chunk: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// aiserver.v1.DiffHistoryData
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffHistoryData {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub diffs: ::prost::alloc::vec::Vec<ComposerFileDiff>,
    #[prost(double, tag = "3")]
    pub timestamp: f64,
    #[prost(string, tag = "4")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub start_to_end_diff: ::core::option::Option<ComposerFileDiff>,
}
/// aiserver.v1.ComposerFileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerFileDiff {
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<composer_file_diff::ChunkDiff>,
    #[prost(enumeration = "composer_file_diff::Editor", tag = "2")]
    pub editor: i32,
    #[prost(bool, tag = "3")]
    pub hit_timeout: bool,
}
/// Nested message and enum types in `ComposerFileDiff`.
pub mod composer_file_diff {
    /// aiserver.v1.ComposerFileDiff.ChunkDiff
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChunkDiff {
        #[prost(string, tag = "1")]
        pub diff_string: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub old_start: i32,
        #[prost(int32, tag = "3")]
        pub new_start: i32,
        #[prost(int32, tag = "4")]
        pub old_lines: i32,
        #[prost(int32, tag = "5")]
        pub new_lines: i32,
        #[prost(int32, tag = "6")]
        pub lines_removed: i32,
        #[prost(int32, tag = "7")]
        pub lines_added: i32,
    }
    /// aiserver.v1.ComposerFileDiff.Editor
    #[allow(clippy::enum_variant_names)]
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
    pub enum Editor {
        Unspecified = 0,
        Ai = 1,
        Human = 2,
    }
    impl Editor {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "EDITOR_UNSPECIFIED",
                Self::Ai => "EDITOR_AI",
                Self::Human => "EDITOR_HUMAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EDITOR_UNSPECIFIED" => Some(Self::Unspecified),
                "EDITOR_AI" => Some(Self::Ai),
                "EDITOR_HUMAN" => Some(Self::Human),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.ConversationSummary
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationSummary {
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub truncation_last_bubble_id_inclusive: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub client_should_start_sending_from_inclusive_bubble_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub previous_conversation_summary_bubble_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub includes_tool_results: bool,
}
/// aiserver.v1.ComposerFileDiffHistory
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerFileDiffHistory {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub diff_history: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(double, repeated, tag = "3")]
    pub diff_history_timestamps: ::prost::alloc::vec::Vec<f64>,
}
/// aiserver.v1.ComposerCapabilityRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerCapabilityRequest {
    #[prost(
        enumeration = "composer_capability_request::ComposerCapabilityType",
        tag = "1"
    )]
    pub r#type: i32,
    #[prost(
        oneof = "composer_capability_request::Data",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14"
    )]
    pub data: ::core::option::Option<composer_capability_request::Data>,
}
/// Nested message and enum types in `ComposerCapabilityRequest`.
pub mod composer_capability_request {
    /// aiserver.v1.ComposerCapabilityRequest.ToolSchema
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolSchema {
        #[prost(enumeration = "ToolType", tag = "1")]
        pub r#type: i32,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(map = "string, message", tag = "3")]
        pub properties: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            SchemaProperty,
        >,
        #[prost(string, repeated, tag = "4")]
        pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.SchemaProperty
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaProperty {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "2")]
        pub description: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.LoopOnLintsCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoopOnLintsCapability {
        #[prost(message, repeated, tag = "1")]
        pub linter_errors: ::prost::alloc::vec::Vec<super::LinterErrors>,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.LoopOnTestsCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoopOnTestsCapability {
        #[prost(string, repeated, tag = "1")]
        pub test_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.MegaPlannerCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MegaPlannerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.LoopOnCommandCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoopOnCommandCapability {
        #[prost(string, tag = "1")]
        pub command: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub output: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "4")]
        pub exit_code: ::core::option::Option<i32>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.ToolCallCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolCallCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub tool_schemas: ::prost::alloc::vec::Vec<ToolSchema>,
        #[prost(string, repeated, tag = "3")]
        pub relevant_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "4")]
        pub files_in_context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "5")]
        pub semantic_search_files: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiffReviewCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub diffs: ::prost::alloc::vec::Vec<diff_review_capability::SimpleFileDiff>,
    }
    /// Nested message and enum types in `DiffReviewCapability`.
    pub mod diff_review_capability {
        /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleFileDiff {
            #[prost(string, tag = "1")]
            pub relative_workspace_path: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "3")]
            pub chunks: ::prost::alloc::vec::Vec<simple_file_diff::Chunk>,
        }
        /// Nested message and enum types in `SimpleFileDiff`.
        pub mod simple_file_diff {
            /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff.Chunk
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Chunk {
                #[prost(string, repeated, tag = "1")]
                pub old_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                #[prost(string, repeated, tag = "2")]
                pub new_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                #[prost(message, optional, tag = "3")]
                pub old_range: ::core::option::Option<super::super::super::LineRange>,
                #[prost(message, optional, tag = "4")]
                pub new_range: ::core::option::Option<super::super::super::LineRange>,
            }
        }
    }
    /// aiserver.v1.ComposerCapabilityRequest.DecomposerCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DecomposerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.ContextPickingCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContextPickingCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "2")]
        pub potential_context_files: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(message, repeated, tag = "3")]
        pub potential_context_code_chunks: ::prost::alloc::vec::Vec<super::CodeChunk>,
        #[prost(string, repeated, tag = "4")]
        pub files_in_context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.EditTrailCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditTrailCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.AutoContextCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutoContextCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "2")]
        pub additional_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.ContextPlannerCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContextPlannerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub attached_code_chunks: ::prost::alloc::vec::Vec<super::CodeChunk>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.RememberThisCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RememberThisCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub memory: ::prost::alloc::string::String,
    }
    /// aiserver.v1.ComposerCapabilityRequest.CursorRulesCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CursorRulesCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ComposerCapabilityRequest.ComposerCapabilityType
    #[allow(clippy::enum_variant_names)]
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
    pub enum ComposerCapabilityType {
        Unspecified = 0,
        LoopOnLints = 1,
        LoopOnTests = 2,
        MegaPlanner = 3,
        LoopOnCommand = 4,
        ToolCall = 5,
        DiffReview = 6,
        ContextPicking = 7,
        EditTrail = 8,
        AutoContext = 9,
        ContextPlanner = 10,
        DiffHistory = 11,
        RememberThis = 12,
        Decomposer = 13,
        UsesCodebase = 14,
        ToolFormer = 15,
        CursorRules = 16,
    }
    impl ComposerCapabilityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "COMPOSER_CAPABILITY_TYPE_UNSPECIFIED",
                Self::LoopOnLints => "COMPOSER_CAPABILITY_TYPE_LOOP_ON_LINTS",
                Self::LoopOnTests => "COMPOSER_CAPABILITY_TYPE_LOOP_ON_TESTS",
                Self::MegaPlanner => "COMPOSER_CAPABILITY_TYPE_MEGA_PLANNER",
                Self::LoopOnCommand => "COMPOSER_CAPABILITY_TYPE_LOOP_ON_COMMAND",
                Self::ToolCall => "COMPOSER_CAPABILITY_TYPE_TOOL_CALL",
                Self::DiffReview => "COMPOSER_CAPABILITY_TYPE_DIFF_REVIEW",
                Self::ContextPicking => "COMPOSER_CAPABILITY_TYPE_CONTEXT_PICKING",
                Self::EditTrail => "COMPOSER_CAPABILITY_TYPE_EDIT_TRAIL",
                Self::AutoContext => "COMPOSER_CAPABILITY_TYPE_AUTO_CONTEXT",
                Self::ContextPlanner => "COMPOSER_CAPABILITY_TYPE_CONTEXT_PLANNER",
                Self::DiffHistory => "COMPOSER_CAPABILITY_TYPE_DIFF_HISTORY",
                Self::RememberThis => "COMPOSER_CAPABILITY_TYPE_REMEMBER_THIS",
                Self::Decomposer => "COMPOSER_CAPABILITY_TYPE_DECOMPOSER",
                Self::UsesCodebase => "COMPOSER_CAPABILITY_TYPE_USES_CODEBASE",
                Self::ToolFormer => "COMPOSER_CAPABILITY_TYPE_TOOL_FORMER",
                Self::CursorRules => "COMPOSER_CAPABILITY_TYPE_CURSOR_RULES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPOSER_CAPABILITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "COMPOSER_CAPABILITY_TYPE_LOOP_ON_LINTS" => Some(Self::LoopOnLints),
                "COMPOSER_CAPABILITY_TYPE_LOOP_ON_TESTS" => Some(Self::LoopOnTests),
                "COMPOSER_CAPABILITY_TYPE_MEGA_PLANNER" => Some(Self::MegaPlanner),
                "COMPOSER_CAPABILITY_TYPE_LOOP_ON_COMMAND" => Some(Self::LoopOnCommand),
                "COMPOSER_CAPABILITY_TYPE_TOOL_CALL" => Some(Self::ToolCall),
                "COMPOSER_CAPABILITY_TYPE_DIFF_REVIEW" => Some(Self::DiffReview),
                "COMPOSER_CAPABILITY_TYPE_CONTEXT_PICKING" => Some(Self::ContextPicking),
                "COMPOSER_CAPABILITY_TYPE_EDIT_TRAIL" => Some(Self::EditTrail),
                "COMPOSER_CAPABILITY_TYPE_AUTO_CONTEXT" => Some(Self::AutoContext),
                "COMPOSER_CAPABILITY_TYPE_CONTEXT_PLANNER" => Some(Self::ContextPlanner),
                "COMPOSER_CAPABILITY_TYPE_DIFF_HISTORY" => Some(Self::DiffHistory),
                "COMPOSER_CAPABILITY_TYPE_REMEMBER_THIS" => Some(Self::RememberThis),
                "COMPOSER_CAPABILITY_TYPE_DECOMPOSER" => Some(Self::Decomposer),
                "COMPOSER_CAPABILITY_TYPE_USES_CODEBASE" => Some(Self::UsesCodebase),
                "COMPOSER_CAPABILITY_TYPE_TOOL_FORMER" => Some(Self::ToolFormer),
                "COMPOSER_CAPABILITY_TYPE_CURSOR_RULES" => Some(Self::CursorRules),
                _ => None,
            }
        }
    }
    /// aiserver.v1.ComposerCapabilityRequest.ToolType
    #[allow(clippy::enum_variant_names)]
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
    pub enum ToolType {
        Unspecified = 0,
        AddFileToContext = 1,
        RunTerminalCommand = 2,
        Iterate = 3,
        RemoveFileFromContext = 4,
        SemanticSearchCodebase = 5,
    }
    impl ToolType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TOOL_TYPE_UNSPECIFIED",
                Self::AddFileToContext => "TOOL_TYPE_ADD_FILE_TO_CONTEXT",
                Self::RunTerminalCommand => "TOOL_TYPE_RUN_TERMINAL_COMMAND",
                Self::Iterate => "TOOL_TYPE_ITERATE",
                Self::RemoveFileFromContext => "TOOL_TYPE_REMOVE_FILE_FROM_CONTEXT",
                Self::SemanticSearchCodebase => "TOOL_TYPE_SEMANTIC_SEARCH_CODEBASE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TOOL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TOOL_TYPE_ADD_FILE_TO_CONTEXT" => Some(Self::AddFileToContext),
                "TOOL_TYPE_RUN_TERMINAL_COMMAND" => Some(Self::RunTerminalCommand),
                "TOOL_TYPE_ITERATE" => Some(Self::Iterate),
                "TOOL_TYPE_REMOVE_FILE_FROM_CONTEXT" => Some(Self::RemoveFileFromContext),
                "TOOL_TYPE_SEMANTIC_SEARCH_CODEBASE" => {
                    Some(Self::SemanticSearchCodebase)
                }
                _ => None,
            }
        }
    }
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "2")]
        LoopOnLints(LoopOnLintsCapability),
        #[prost(message, tag = "3")]
        LoopOnTests(LoopOnTestsCapability),
        #[prost(message, tag = "4")]
        MegaPlanner(MegaPlannerCapability),
        #[prost(message, tag = "5")]
        LoopOnCommand(LoopOnCommandCapability),
        #[prost(message, tag = "6")]
        ToolCall(ToolCallCapability),
        #[prost(message, tag = "7")]
        DiffReview(DiffReviewCapability),
        #[prost(message, tag = "8")]
        ContextPicking(ContextPickingCapability),
        #[prost(message, tag = "9")]
        EditTrail(EditTrailCapability),
        #[prost(message, tag = "10")]
        AutoContext(AutoContextCapability),
        #[prost(message, tag = "11")]
        ContextPlanner(ContextPlannerCapability),
        #[prost(message, tag = "12")]
        RememberThis(RememberThisCapability),
        #[prost(message, tag = "13")]
        Decomposer(DecomposerCapability),
        #[prost(message, tag = "14")]
        CursorRules(CursorRulesCapability),
    }
}
/// aiserver.v1.CodeChunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeChunk {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub start_line_number: i32,
    #[prost(string, repeated, tag = "3")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "code_chunk::SummarizationStrategy", optional, tag = "4")]
    pub summarization_strategy: ::core::option::Option<i32>,
    #[prost(string, tag = "5")]
    pub language_identifier: ::prost::alloc::string::String,
    #[prost(enumeration = "code_chunk::Intent", optional, tag = "6")]
    pub intent: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_final_version: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_first_version: ::core::option::Option<bool>,
}
/// Nested message and enum types in `CodeChunk`.
pub mod code_chunk {
    /// aiserver.v1.CodeChunk.Intent
    #[allow(clippy::enum_variant_names)]
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
    pub enum Intent {
        Unspecified = 0,
        ComposerFile = 1,
        CompressedComposerFile = 2,
    }
    impl Intent {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "INTENT_UNSPECIFIED",
                Self::ComposerFile => "INTENT_COMPOSER_FILE",
                Self::CompressedComposerFile => "INTENT_COMPRESSED_COMPOSER_FILE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTENT_UNSPECIFIED" => Some(Self::Unspecified),
                "INTENT_COMPOSER_FILE" => Some(Self::ComposerFile),
                "INTENT_COMPRESSED_COMPOSER_FILE" => Some(Self::CompressedComposerFile),
                _ => None,
            }
        }
    }
    /// aiserver.v1.CodeChunk.SummarizationStrategy
    #[allow(clippy::enum_variant_names)]
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
    pub enum SummarizationStrategy {
        NoneUnspecified = 0,
        Summarized = 1,
        Embedded = 2,
    }
    impl SummarizationStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::NoneUnspecified => "SUMMARIZATION_STRATEGY_NONE_UNSPECIFIED",
                Self::Summarized => "SUMMARIZATION_STRATEGY_SUMMARIZED",
                Self::Embedded => "SUMMARIZATION_STRATEGY_EMBEDDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUMMARIZATION_STRATEGY_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
                "SUMMARIZATION_STRATEGY_SUMMARIZED" => Some(Self::Summarized),
                "SUMMARIZATION_STRATEGY_EMBEDDED" => Some(Self::Embedded),
                _ => None,
            }
        }
    }
}
/// aiserver.v1.Commit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(string, tag = "1")]
    pub sha: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub diff: ::prost::alloc::vec::Vec<FileDiff>,
    #[prost(string, tag = "5")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub date: ::prost::alloc::string::String,
}
/// aiserver.v1.ClientSideToolV2Result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSideToolV2Result {
    #[prost(enumeration = "ClientSideToolV2", tag = "1")]
    pub tool: i32,
    #[prost(message, optional, tag = "8")]
    pub error: ::core::option::Option<ToolResultError>,
    #[prost(
        oneof = "client_side_tool_v2_result::Result",
        tags = "2, 3, 4, 5, 6, 9, 10, 11, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30"
    )]
    pub result: ::core::option::Option<client_side_tool_v2_result::Result>,
}
/// Nested message and enum types in `ClientSideToolV2Result`.
pub mod client_side_tool_v2_result {
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        ReadSemsearchFilesResult(super::ReadSemsearchFilesResult),
        #[prost(message, tag = "3")]
        ReadFileForImportsResult(super::ReadFileForImportsResult),
        #[prost(message, tag = "4")]
        RipgrepSearchResult(super::RipgrepSearchResult),
        #[prost(message, tag = "5")]
        RunTerminalCommandResult(super::RunTerminalCommandResult),
        #[prost(message, tag = "6")]
        ReadFileResult(super::ReadFileResult),
        #[prost(message, tag = "9")]
        ListDirResult(super::ListDirResult),
        #[prost(message, tag = "10")]
        EditFileResult(super::EditFileResult),
        #[prost(message, tag = "11")]
        FileSearchResult(super::ToolCallFileSearchResult),
        #[prost(message, tag = "18")]
        SemanticSearchFullResult(super::SemanticSearchFullResult),
        #[prost(message, tag = "19")]
        CreateFileResult(super::CreateFileResult),
        #[prost(message, tag = "20")]
        DeleteFileResult(super::DeleteFileResult),
        #[prost(message, tag = "21")]
        ReapplyResult(super::ReapplyResult),
        #[prost(message, tag = "22")]
        GetRelatedFilesResult(super::GetRelatedFilesResult),
        #[prost(message, tag = "23")]
        ParallelApplyResult(super::ParallelApplyResult),
        #[prost(message, tag = "24")]
        RunTerminalCommandV2Result(super::RunTerminalCommandV2Result),
        #[prost(message, tag = "25")]
        FetchRulesResult(super::FetchRulesResult),
        #[prost(message, tag = "26")]
        PlannerResult(super::PlannerResult),
        #[prost(message, tag = "27")]
        WebSearchResult(super::WebSearchResult),
        #[prost(message, tag = "28")]
        McpResult(super::McpResult),
        #[prost(message, tag = "29")]
        WebViewerResult(super::WebViewerResult),
        #[prost(message, tag = "30")]
        DiffHistoryResult(super::DiffHistoryResult),
    }
}
/// aiserver.v1.WebViewerResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebViewerResult {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub screenshot: ::core::option::Option<ImageProto>,
}
/// aiserver.v1.WebSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSearchResult {
    #[prost(message, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<web_search_result::WebReference>,
}
/// Nested message and enum types in `WebSearchResult`.
pub mod web_search_result {
    /// aiserver.v1.WebSearchResult.WebReference
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebReference {
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub url: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub chunk: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.ToolCallFileSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolCallFileSearchResult {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<tool_call_file_search_result::File>,
    #[prost(bool, optional, tag = "2")]
    pub limit_hit: ::core::option::Option<bool>,
    #[prost(int32, tag = "3")]
    pub num_results: i32,
}
/// Nested message and enum types in `ToolCallFileSearchResult`.
pub mod tool_call_file_search_result {
    /// aiserver.v1.ToolCallFileSearchResult.File
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// aiserver.v1.SemanticSearchFullResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticSearchFullResult {
    #[prost(message, repeated, tag = "1")]
    pub code_results: ::prost::alloc::vec::Vec<CodeResult>,
    #[prost(message, repeated, tag = "2")]
    pub all_files: ::prost::alloc::vec::Vec<File>,
    #[prost(message, repeated, tag = "3")]
    pub missing_files: ::prost::alloc::vec::Vec<MissingFile>,
}
/// aiserver.v1.RunTerminalCommandV2Result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTerminalCommandV2Result {
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
    #[prost(bool, optional, tag = "3")]
    pub rejected: ::core::option::Option<bool>,
    #[prost(bool, tag = "4")]
    pub popped_out_into_background: bool,
    #[prost(bool, tag = "5")]
    pub is_running_in_background: bool,
    #[prost(bool, tag = "6")]
    pub not_interrupted: bool,
    #[prost(string, tag = "7")]
    pub resulting_working_directory: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub did_user_change: bool,
    #[prost(enumeration = "RunTerminalCommandEndedReason", tag = "9")]
    pub ended_reason: i32,
    #[prost(int32, optional, tag = "10")]
    pub exit_code_v2: ::core::option::Option<i32>,
}
/// aiserver.v1.RunTerminalCommandResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTerminalCommandResult {
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
    #[prost(bool, optional, tag = "3")]
    pub rejected: ::core::option::Option<bool>,
    #[prost(bool, tag = "4")]
    pub popped_out_into_background: bool,
}
/// aiserver.v1.RipgrepSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipgrepSearchResult {
    #[prost(message, optional, tag = "1")]
    pub internal: ::core::option::Option<RipgrepSearchResultInternal>,
}
/// aiserver.v1.RipgrepSearchResultInternal
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipgrepSearchResultInternal {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<ripgrep_search_result_internal::IFileMatch>,
    #[prost(
        enumeration = "ripgrep_search_result_internal::SearchCompletionExitCode",
        optional,
        tag = "2"
    )]
    pub exit: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub limit_hit: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub messages: ::prost::alloc::vec::Vec<
        ripgrep_search_result_internal::ITextSearchCompleteMessage,
    >,
    #[prost(oneof = "ripgrep_search_result_internal::Stats", tags = "5, 6")]
    pub stats: ::core::option::Option<ripgrep_search_result_internal::Stats>,
}
/// Nested message and enum types in `RipgrepSearchResultInternal`.
pub mod ripgrep_search_result_internal {
    /// aiserver.v1.RipgrepSearchResultInternal.IFileMatch
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IFileMatch {
        #[prost(string, tag = "1")]
        pub resource: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub results: ::prost::alloc::vec::Vec<ITextSearchResult>,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchResult
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ITextSearchResult {
        #[prost(oneof = "i_text_search_result::Result", tags = "1, 2")]
        pub result: ::core::option::Option<i_text_search_result::Result>,
    }
    /// Nested message and enum types in `ITextSearchResult`.
    pub mod i_text_search_result {
        #[allow(clippy::enum_variant_names)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            #[prost(message, tag = "1")]
            Match(super::ITextSearchMatch),
            #[prost(message, tag = "2")]
            Context(super::ITextSearchContext),
        }
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchMatch
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ITextSearchMatch {
        #[prost(string, optional, tag = "1")]
        pub uri: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub range_locations: ::prost::alloc::vec::Vec<ISearchRangeSetPairing>,
        #[prost(string, tag = "3")]
        pub preview_text: ::prost::alloc::string::String,
        #[prost(int32, optional, tag = "4")]
        pub webview_index: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "5")]
        pub cell_fragment: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchContext
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ITextSearchContext {
        #[prost(string, optional, tag = "1")]
        pub uri: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub line_number: i32,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ISearchRangeSetPairing
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ISearchRangeSetPairing {
        #[prost(message, optional, tag = "1")]
        pub source: ::core::option::Option<ISearchRange>,
        #[prost(message, optional, tag = "2")]
        pub preview: ::core::option::Option<ISearchRange>,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ISearchRange
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ISearchRange {
        #[prost(int32, tag = "1")]
        pub start_line_number: i32,
        #[prost(int32, tag = "2")]
        pub start_column: i32,
        #[prost(int32, tag = "3")]
        pub end_line_number: i32,
        #[prost(int32, tag = "4")]
        pub end_column: i32,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchCompleteMessage
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ITextSearchCompleteMessage {
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
        #[prost(enumeration = "TextSearchCompleteMessageType", tag = "2")]
        pub r#type: i32,
        #[prost(bool, optional, tag = "3")]
        pub trusted: ::core::option::Option<bool>,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchStats
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct IFileSearchStats {
        #[prost(bool, tag = "1")]
        pub from_cache: bool,
        #[prost(int32, tag = "5")]
        pub result_count: i32,
        #[prost(enumeration = "i_file_search_stats::FileSearchProviderType", tag = "6")]
        pub r#type: i32,
        #[prost(int32, optional, tag = "7")]
        pub sorting_time: ::core::option::Option<i32>,
        #[prost(oneof = "i_file_search_stats::DetailStats", tags = "2, 3, 4")]
        pub detail_stats: ::core::option::Option<i_file_search_stats::DetailStats>,
    }
    /// Nested message and enum types in `IFileSearchStats`.
    pub mod i_file_search_stats {
        /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchStats.FileSearchProviderType
        #[allow(clippy::enum_variant_names)]
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
        pub enum FileSearchProviderType {
            Unspecified = 0,
            FileSearchProvider = 1,
            SearchProcess = 2,
        }
        impl FileSearchProviderType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "FILE_SEARCH_PROVIDER_TYPE_UNSPECIFIED",
                    Self::FileSearchProvider => {
                        "FILE_SEARCH_PROVIDER_TYPE_FILE_SEARCH_PROVIDER"
                    }
                    Self::SearchProcess => "FILE_SEARCH_PROVIDER_TYPE_SEARCH_PROCESS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "FILE_SEARCH_PROVIDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "FILE_SEARCH_PROVIDER_TYPE_FILE_SEARCH_PROVIDER" => {
                        Some(Self::FileSearchProvider)
                    }
                    "FILE_SEARCH_PROVIDER_TYPE_SEARCH_PROCESS" => {
                        Some(Self::SearchProcess)
                    }
                    _ => None,
                }
            }
        }
        #[allow(clippy::enum_variant_names)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum DetailStats {
            #[prost(message, tag = "2")]
            SearchEngineStats(super::ISearchEngineStats),
            #[prost(message, tag = "3")]
            CachedSearchStats(super::ICachedSearchStats),
            #[prost(message, tag = "4")]
            FileSearchProviderStats(super::IFileSearchProviderStats),
        }
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchStats
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ITextSearchStats {
        #[prost(enumeration = "i_text_search_stats::TextSearchProviderType", tag = "1")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `ITextSearchStats`.
    pub mod i_text_search_stats {
        /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchStats.TextSearchProviderType
        #[allow(clippy::enum_variant_names)]
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
        pub enum TextSearchProviderType {
            Unspecified = 0,
            TextSearchProvider = 1,
            SearchProcess = 2,
            AiTextSearchProvider = 3,
        }
        impl TextSearchProviderType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "TEXT_SEARCH_PROVIDER_TYPE_UNSPECIFIED",
                    Self::TextSearchProvider => {
                        "TEXT_SEARCH_PROVIDER_TYPE_TEXT_SEARCH_PROVIDER"
                    }
                    Self::SearchProcess => "TEXT_SEARCH_PROVIDER_TYPE_SEARCH_PROCESS",
                    Self::AiTextSearchProvider => {
                        "TEXT_SEARCH_PROVIDER_TYPE_AI_TEXT_SEARCH_PROVIDER"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TEXT_SEARCH_PROVIDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "TEXT_SEARCH_PROVIDER_TYPE_TEXT_SEARCH_PROVIDER" => {
                        Some(Self::TextSearchProvider)
                    }
                    "TEXT_SEARCH_PROVIDER_TYPE_SEARCH_PROCESS" => {
                        Some(Self::SearchProcess)
                    }
                    "TEXT_SEARCH_PROVIDER_TYPE_AI_TEXT_SEARCH_PROVIDER" => {
                        Some(Self::AiTextSearchProvider)
                    }
                    _ => None,
                }
            }
        }
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ISearchEngineStats
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ISearchEngineStats {
        #[prost(int32, tag = "1")]
        pub file_walk_time: i32,
        #[prost(int32, tag = "2")]
        pub directories_walked: i32,
        #[prost(int32, tag = "3")]
        pub files_walked: i32,
        #[prost(int32, tag = "4")]
        pub cmd_time: i32,
        #[prost(int32, optional, tag = "5")]
        pub cmd_result_count: ::core::option::Option<i32>,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.ICachedSearchStats
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ICachedSearchStats {
        #[prost(bool, tag = "1")]
        pub cache_was_resolved: bool,
        #[prost(int32, tag = "2")]
        pub cache_lookup_time: i32,
        #[prost(int32, tag = "3")]
        pub cache_filter_time: i32,
        #[prost(int32, tag = "4")]
        pub cache_entry_count: i32,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchProviderStats
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct IFileSearchProviderStats {
        #[prost(int32, tag = "1")]
        pub provider_time: i32,
        #[prost(int32, tag = "2")]
        pub post_process_time: i32,
    }
    /// aiserver.v1.RipgrepSearchResultInternal.TextSearchCompleteMessageType
    #[allow(clippy::enum_variant_names)]
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
    pub enum TextSearchCompleteMessageType {
        Unspecified = 0,
        Information = 1,
        Warning = 2,
    }
    impl TextSearchCompleteMessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_UNSPECIFIED",
                Self::Information => "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_INFORMATION",
                Self::Warning => "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_WARNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_UNSPECIFIED" => {
                    Some(Self::Unspecified)
                }
                "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_INFORMATION" => {
                    Some(Self::Information)
                }
                "TEXT_SEARCH_COMPLETE_MESSAGE_TYPE_WARNING" => Some(Self::Warning),
                _ => None,
            }
        }
    }
    /// aiserver.v1.RipgrepSearchResultInternal.SearchCompletionExitCode
    #[allow(clippy::enum_variant_names)]
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
    pub enum SearchCompletionExitCode {
        Unspecified = 0,
        Normal = 1,
        NewSearchStarted = 2,
    }
    impl SearchCompletionExitCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "SEARCH_COMPLETION_EXIT_CODE_UNSPECIFIED",
                Self::Normal => "SEARCH_COMPLETION_EXIT_CODE_NORMAL",
                Self::NewSearchStarted => {
                    "SEARCH_COMPLETION_EXIT_CODE_NEW_SEARCH_STARTED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEARCH_COMPLETION_EXIT_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "SEARCH_COMPLETION_EXIT_CODE_NORMAL" => Some(Self::Normal),
                "SEARCH_COMPLETION_EXIT_CODE_NEW_SEARCH_STARTED" => {
                    Some(Self::NewSearchStarted)
                }
                _ => None,
            }
        }
    }
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Stats {
        #[prost(message, tag = "5")]
        FileSearchStats(IFileSearchStats),
        #[prost(message, tag = "6")]
        TextSearchStats(ITextSearchStats),
    }
}
/// aiserver.v1.ReapplyResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReapplyResult {
    #[prost(message, optional, tag = "1")]
    pub diff: ::core::option::Option<edit_file_result::FileDiff>,
    #[prost(bool, tag = "2")]
    pub is_applied: bool,
    #[prost(bool, tag = "3")]
    pub apply_failed: bool,
    #[prost(message, repeated, tag = "4")]
    pub linter_errors: ::prost::alloc::vec::Vec<LinterError>,
}
/// aiserver.v1.ReadFileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileResult {
    #[prost(string, tag = "1")]
    pub contents: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub did_downgrade_to_line_range: bool,
    #[prost(bool, tag = "3")]
    pub did_shorten_line_range: bool,
    #[prost(bool, tag = "4")]
    pub did_set_default_line_range: bool,
    #[prost(string, optional, tag = "5")]
    pub full_file_contents: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub outline: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub start_line_one_indexed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub end_line_one_indexed_inclusive: ::core::option::Option<i32>,
    #[prost(string, tag = "9")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub did_shorten_char_range: bool,
}
/// aiserver.v1.ReadFileForImportsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileForImportsResult {
    #[prost(string, tag = "1")]
    pub contents: ::prost::alloc::string::String,
}
/// aiserver.v1.PlannerResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannerResult {
    #[prost(string, tag = "1")]
    pub plan: ::prost::alloc::string::String,
}
/// aiserver.v1.ParallelApplyResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParallelApplyResult {
    #[prost(message, repeated, tag = "1")]
    pub file_results: ::prost::alloc::vec::Vec<parallel_apply_result::FileResult>,
    #[prost(string, optional, tag = "2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ParallelApplyResult`.
pub mod parallel_apply_result {
    /// aiserver.v1.ParallelApplyResult.FileResult
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FileResult {
        #[prost(string, tag = "1")]
        pub file_path: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub diff: ::core::option::Option<super::edit_file_result::FileDiff>,
        #[prost(bool, tag = "3")]
        pub is_applied: bool,
        #[prost(bool, tag = "4")]
        pub apply_failed: bool,
        #[prost(string, optional, tag = "5")]
        pub error: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "6")]
        pub linter_errors: ::prost::alloc::vec::Vec<super::LinterError>,
    }
}
/// aiserver.v1.FileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileResult {
    #[prost(message, optional, tag = "1")]
    pub file: ::core::option::Option<File>,
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// aiserver.v1.MCPResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct McpResult {
    #[prost(string, tag = "1")]
    pub selected_tool: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub result: ::prost::alloc::string::String,
}
/// aiserver.v1.GetRelatedFilesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelatedFilesResult {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<get_related_files_result::File>,
}
/// Nested message and enum types in `GetRelatedFilesResult`.
pub mod get_related_files_result {
    /// aiserver.v1.GetRelatedFilesResult.File
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        #[prost(float, tag = "2")]
        pub score: f32,
    }
}
/// aiserver.v1.FetchRulesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRulesResult {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<CursorRule>,
}
/// aiserver.v1.EditFileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFileResult {
    #[prost(message, optional, tag = "1")]
    pub diff: ::core::option::Option<edit_file_result::FileDiff>,
    #[prost(bool, tag = "2")]
    pub is_applied: bool,
    #[prost(bool, tag = "3")]
    pub apply_failed: bool,
    #[prost(message, repeated, tag = "4")]
    pub linter_errors: ::prost::alloc::vec::Vec<LinterError>,
}
/// Nested message and enum types in `EditFileResult`.
pub mod edit_file_result {
    /// aiserver.v1.EditFileResult.FileDiff
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FileDiff {
        #[prost(message, repeated, tag = "1")]
        pub chunks: ::prost::alloc::vec::Vec<file_diff::ChunkDiff>,
        #[prost(enumeration = "file_diff::Editor", tag = "2")]
        pub editor: i32,
        #[prost(bool, tag = "3")]
        pub hit_timeout: bool,
    }
    /// Nested message and enum types in `FileDiff`.
    pub mod file_diff {
        /// aiserver.v1.EditFileResult.FileDiff.ChunkDiff
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ChunkDiff {
            #[prost(string, tag = "1")]
            pub diff_string: ::prost::alloc::string::String,
            #[prost(int32, tag = "2")]
            pub old_start: i32,
            #[prost(int32, tag = "3")]
            pub new_start: i32,
            #[prost(int32, tag = "4")]
            pub old_lines: i32,
            #[prost(int32, tag = "5")]
            pub new_lines: i32,
            #[prost(int32, tag = "6")]
            pub lines_removed: i32,
            #[prost(int32, tag = "7")]
            pub lines_added: i32,
        }
        /// aiserver.v1.EditFileResult.FileDiff.Editor
        #[allow(clippy::enum_variant_names)]
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
        pub enum Editor {
            Unspecified = 0,
            Ai = 1,
            Human = 2,
        }
        impl Editor {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "EDITOR_UNSPECIFIED",
                    Self::Ai => "EDITOR_AI",
                    Self::Human => "EDITOR_HUMAN",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "EDITOR_UNSPECIFIED" => Some(Self::Unspecified),
                    "EDITOR_AI" => Some(Self::Ai),
                    "EDITOR_HUMAN" => Some(Self::Human),
                    _ => None,
                }
            }
        }
    }
}
/// aiserver.v1.DiffHistoryResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffHistoryResult {
    #[prost(message, repeated, tag = "40")]
    pub human_changes: ::prost::alloc::vec::Vec<diff_history_result::HumanChange>,
}
/// Nested message and enum types in `DiffHistoryResult`.
pub mod diff_history_result {
    /// aiserver.v1.DiffHistoryResult.RenderedDiff
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RenderedDiff {
        #[prost(int32, tag = "1")]
        pub start_line_number: i32,
        #[prost(int32, tag = "2")]
        pub end_line_number_exclusive: i32,
        #[prost(string, repeated, tag = "3")]
        pub before_context_lines: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, repeated, tag = "4")]
        pub removed_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "5")]
        pub added_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "6")]
        pub after_context_lines: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// aiserver.v1.DiffHistoryResult.HumanChange
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HumanChange {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub rendered_diffs: ::prost::alloc::vec::Vec<RenderedDiff>,
    }
}
/// aiserver.v1.ContextAST
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextAst {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<ContainerTree>,
}
/// aiserver.v1.ContainerTree
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerTree {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<ContainerTreeNode>,
}
/// aiserver.v1.ContainerTreeNode
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerTreeNode {
    #[prost(oneof = "container_tree_node::Node", tags = "1, 2, 3")]
    pub node: ::core::option::Option<container_tree_node::Node>,
}
/// Nested message and enum types in `ContainerTreeNode`.
pub mod container_tree_node {
    /// aiserver.v1.ContainerTreeNode.Symbol
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Symbol {
        #[prost(string, tag = "1")]
        pub doc_string: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "6")]
        pub references: ::prost::alloc::vec::Vec<Reference>,
        #[prost(double, tag = "7")]
        pub score: f64,
    }
    /// aiserver.v1.ContainerTreeNode.Container
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Container {
        #[prost(string, tag = "1")]
        pub doc_string: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub header: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub trailer: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub children: ::prost::alloc::vec::Vec<super::ContainerTreeNode>,
        #[prost(message, repeated, tag = "6")]
        pub references: ::prost::alloc::vec::Vec<Reference>,
        #[prost(double, tag = "7")]
        pub score: f64,
    }
    /// aiserver.v1.ContainerTreeNode.Blob
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Blob {
        #[prost(string, optional, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// aiserver.v1.ContainerTreeNode.Reference
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reference {
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub relative_workspace_path: ::prost::alloc::string::String,
    }
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Node {
        #[prost(message, tag = "1")]
        Container(Container),
        #[prost(message, tag = "2")]
        Blob(Blob),
        #[prost(message, tag = "3")]
        Symbol(Symbol),
    }
}
/// aiserver.v1.CommitNote
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitNote {
    #[prost(string, tag = "1")]
    pub note: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub commit_hash: ::prost::alloc::string::String,
}
/// aiserver.v1.ChatQuote
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatQuote {
    #[prost(string, tag = "1")]
    pub markdown: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bubble_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub section_index: i32,
}
/// aiserver.v1.ChatExternalLink
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatExternalLink {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub uuid: ::prost::alloc::string::String,
}
/// aiserver.v1.StreamChatResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamChatResponse {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "22")]
    pub server_bubble_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub debugging_only_chat_prompt: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(int32, optional, tag = "3")]
    pub debugging_only_token_count: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub document_citation: ::core::option::Option<DocumentationCitation>,
    #[prost(string, optional, tag = "5")]
    pub filled_prompt: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub is_big_file: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub intermediate_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "10")]
    pub is_using_slow_request: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "8")]
    pub chunk_identity: ::core::option::Option<stream_chat_response::ChunkIdentity>,
    #[prost(message, optional, tag = "9")]
    pub docs_reference: ::core::option::Option<DocsReference>,
    #[prost(message, optional, tag = "11")]
    pub web_citation: ::core::option::Option<WebCitation>,
    #[prost(message, optional, tag = "12")]
    pub status_updates: ::core::option::Option<StatusUpdates>,
    #[prost(message, optional, tag = "13")]
    pub timing_info: ::core::option::Option<ServerTimingInfo>,
    #[prost(message, optional, tag = "14")]
    pub symbol_link: ::core::option::Option<SymbolLink>,
    #[prost(message, optional, tag = "15")]
    pub file_link: ::core::option::Option<FileLink>,
    #[prost(message, optional, tag = "16")]
    pub conversation_summary: ::core::option::Option<ConversationSummary>,
    #[prost(message, optional, tag = "17")]
    pub service_status_update: ::core::option::Option<ServiceStatusUpdate>,
}
/// Nested message and enum types in `StreamChatResponse`.
pub mod stream_chat_response {
    /// aiserver.v1.StreamChatResponse.ChunkIdentity
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChunkIdentity {
        #[prost(string, tag = "1")]
        pub file_name: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub start_line: i32,
        #[prost(int32, tag = "3")]
        pub end_line: i32,
        #[prost(string, tag = "4")]
        pub text: ::prost::alloc::string::String,
        #[prost(enumeration = "super::ChunkType", tag = "5")]
        pub chunk_type: i32,
    }
}
/// aiserver.v1.WebCitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebCitation {
    #[prost(message, repeated, tag = "1")]
    pub references: ::prost::alloc::vec::Vec<WebReference>,
}
/// aiserver.v1.SymbolLink
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolLink {
    #[prost(string, tag = "1")]
    pub symbol_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol_search_string: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub relative_workspace_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub rough_line_number: i32,
}
/// aiserver.v1.StatusUpdates
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdates {
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<StatusUpdate>,
}
/// aiserver.v1.StatusUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdate {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
}
/// aiserver.v1.ServiceStatusUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceStatusUpdate {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub codicon: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "3")]
    pub allow_command_links_potentially_unsafe_please_only_use_for_handwritten_trusted_markdown: ::core::option::Option<
        bool,
    >,
    #[prost(string, optional, tag = "4")]
    pub action_to_run_on_status_update: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// aiserver.v1.ServerTimingInfo
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerTimingInfo {
    #[prost(double, tag = "1")]
    pub server_start_time: f64,
    #[prost(double, tag = "2")]
    pub server_first_token_time: f64,
    #[prost(double, tag = "3")]
    pub server_request_sent_time: f64,
    #[prost(double, tag = "4")]
    pub server_end_time: f64,
}
/// aiserver.v1.FileLink
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileLink {
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub relative_workspace_path: ::prost::alloc::string::String,
}
/// aiserver.v1.DocumentationCitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationCitation {
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<DocumentationChunk>,
}
/// aiserver.v1.DocumentationChunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationChunk {
    #[prost(string, tag = "1")]
    pub doc_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub page_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub documentation_chunk: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub score: f32,
    #[prost(string, tag = "5")]
    pub page_title: ::prost::alloc::string::String,
}
/// aiserver.v1.EmbeddingModel
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmbeddingModel {
    Unspecified = 0,
    VoyageCode2 = 1,
    TextEmbeddingsLarge3 = 2,
    Qwen15bCustom = 3,
}
impl EmbeddingModel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "EMBEDDING_MODEL_UNSPECIFIED",
            Self::VoyageCode2 => "EMBEDDING_MODEL_VOYAGE_CODE_2",
            Self::TextEmbeddingsLarge3 => "EMBEDDING_MODEL_TEXT_EMBEDDINGS_LARGE_3",
            Self::Qwen15bCustom => "EMBEDDING_MODEL_QWEN_1_5B_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMBEDDING_MODEL_UNSPECIFIED" => Some(Self::Unspecified),
            "EMBEDDING_MODEL_VOYAGE_CODE_2" => Some(Self::VoyageCode2),
            "EMBEDDING_MODEL_TEXT_EMBEDDINGS_LARGE_3" => Some(Self::TextEmbeddingsLarge3),
            "EMBEDDING_MODEL_QWEN_1_5B_CUSTOM" => Some(Self::Qwen15bCustom),
            _ => None,
        }
    }
}
/// aiserver.v1.BuiltinTool
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuiltinTool {
    Unspecified = 0,
    Search = 1,
    ReadChunk = 2,
    Gotodef = 3,
    Edit = 4,
    UndoEdit = 5,
    End = 6,
    NewFile = 7,
    AddTest = 8,
    RunTest = 9,
    DeleteTest = 10,
    SaveFile = 11,
    GetTests = 12,
    GetSymbols = 13,
    SemanticSearch = 14,
    GetProjectStructure = 15,
    CreateRmFiles = 16,
    RunTerminalCommands = 17,
    NewEdit = 18,
    ReadWithLinter = 19,
}
impl BuiltinTool {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "BUILTIN_TOOL_UNSPECIFIED",
            Self::Search => "BUILTIN_TOOL_SEARCH",
            Self::ReadChunk => "BUILTIN_TOOL_READ_CHUNK",
            Self::Gotodef => "BUILTIN_TOOL_GOTODEF",
            Self::Edit => "BUILTIN_TOOL_EDIT",
            Self::UndoEdit => "BUILTIN_TOOL_UNDO_EDIT",
            Self::End => "BUILTIN_TOOL_END",
            Self::NewFile => "BUILTIN_TOOL_NEW_FILE",
            Self::AddTest => "BUILTIN_TOOL_ADD_TEST",
            Self::RunTest => "BUILTIN_TOOL_RUN_TEST",
            Self::DeleteTest => "BUILTIN_TOOL_DELETE_TEST",
            Self::SaveFile => "BUILTIN_TOOL_SAVE_FILE",
            Self::GetTests => "BUILTIN_TOOL_GET_TESTS",
            Self::GetSymbols => "BUILTIN_TOOL_GET_SYMBOLS",
            Self::SemanticSearch => "BUILTIN_TOOL_SEMANTIC_SEARCH",
            Self::GetProjectStructure => "BUILTIN_TOOL_GET_PROJECT_STRUCTURE",
            Self::CreateRmFiles => "BUILTIN_TOOL_CREATE_RM_FILES",
            Self::RunTerminalCommands => "BUILTIN_TOOL_RUN_TERMINAL_COMMANDS",
            Self::NewEdit => "BUILTIN_TOOL_NEW_EDIT",
            Self::ReadWithLinter => "BUILTIN_TOOL_READ_WITH_LINTER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUILTIN_TOOL_UNSPECIFIED" => Some(Self::Unspecified),
            "BUILTIN_TOOL_SEARCH" => Some(Self::Search),
            "BUILTIN_TOOL_READ_CHUNK" => Some(Self::ReadChunk),
            "BUILTIN_TOOL_GOTODEF" => Some(Self::Gotodef),
            "BUILTIN_TOOL_EDIT" => Some(Self::Edit),
            "BUILTIN_TOOL_UNDO_EDIT" => Some(Self::UndoEdit),
            "BUILTIN_TOOL_END" => Some(Self::End),
            "BUILTIN_TOOL_NEW_FILE" => Some(Self::NewFile),
            "BUILTIN_TOOL_ADD_TEST" => Some(Self::AddTest),
            "BUILTIN_TOOL_RUN_TEST" => Some(Self::RunTest),
            "BUILTIN_TOOL_DELETE_TEST" => Some(Self::DeleteTest),
            "BUILTIN_TOOL_SAVE_FILE" => Some(Self::SaveFile),
            "BUILTIN_TOOL_GET_TESTS" => Some(Self::GetTests),
            "BUILTIN_TOOL_GET_SYMBOLS" => Some(Self::GetSymbols),
            "BUILTIN_TOOL_SEMANTIC_SEARCH" => Some(Self::SemanticSearch),
            "BUILTIN_TOOL_GET_PROJECT_STRUCTURE" => Some(Self::GetProjectStructure),
            "BUILTIN_TOOL_CREATE_RM_FILES" => Some(Self::CreateRmFiles),
            "BUILTIN_TOOL_RUN_TERMINAL_COMMANDS" => Some(Self::RunTerminalCommands),
            "BUILTIN_TOOL_NEW_EDIT" => Some(Self::NewEdit),
            "BUILTIN_TOOL_READ_WITH_LINTER" => Some(Self::ReadWithLinter),
            _ => None,
        }
    }
}
/// aiserver.v1.LintSeverity
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LintSeverity {
    Unspecified = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Hint = 4,
    Ai = 5,
}
impl LintSeverity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "LINT_SEVERITY_UNSPECIFIED",
            Self::Error => "LINT_SEVERITY_ERROR",
            Self::Warning => "LINT_SEVERITY_WARNING",
            Self::Info => "LINT_SEVERITY_INFO",
            Self::Hint => "LINT_SEVERITY_HINT",
            Self::Ai => "LINT_SEVERITY_AI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINT_SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
            "LINT_SEVERITY_ERROR" => Some(Self::Error),
            "LINT_SEVERITY_WARNING" => Some(Self::Warning),
            "LINT_SEVERITY_INFO" => Some(Self::Info),
            "LINT_SEVERITY_HINT" => Some(Self::Hint),
            "LINT_SEVERITY_AI" => Some(Self::Ai),
            _ => None,
        }
    }
}
/// aiserver.v1.ClientSideToolV2
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientSideToolV2 {
    Unspecified = 0,
    ReadSemsearchFiles = 1,
    ReadFileForImports = 2,
    RipgrepSearch = 3,
    RunTerminalCommand = 4,
    ReadFile = 5,
    ListDir = 6,
    EditFile = 7,
    FileSearch = 8,
    SemanticSearchFull = 9,
    CreateFile = 10,
    DeleteFile = 11,
    Reapply = 12,
    GetRelatedFiles = 13,
    ParallelApply = 14,
    RunTerminalCommandV2 = 15,
    FetchRules = 16,
    Planner = 17,
    WebSearch = 18,
    Mcp = 19,
    WebViewer = 20,
    DiffHistory = 21,
}
impl ClientSideToolV2 {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "CLIENT_SIDE_TOOL_V2_UNSPECIFIED",
            Self::ReadSemsearchFiles => "CLIENT_SIDE_TOOL_V2_READ_SEMSEARCH_FILES",
            Self::ReadFileForImports => "CLIENT_SIDE_TOOL_V2_READ_FILE_FOR_IMPORTS",
            Self::RipgrepSearch => "CLIENT_SIDE_TOOL_V2_RIPGREP_SEARCH",
            Self::RunTerminalCommand => "CLIENT_SIDE_TOOL_V2_RUN_TERMINAL_COMMAND",
            Self::ReadFile => "CLIENT_SIDE_TOOL_V2_READ_FILE",
            Self::ListDir => "CLIENT_SIDE_TOOL_V2_LIST_DIR",
            Self::EditFile => "CLIENT_SIDE_TOOL_V2_EDIT_FILE",
            Self::FileSearch => "CLIENT_SIDE_TOOL_V2_FILE_SEARCH",
            Self::SemanticSearchFull => "CLIENT_SIDE_TOOL_V2_SEMANTIC_SEARCH_FULL",
            Self::CreateFile => "CLIENT_SIDE_TOOL_V2_CREATE_FILE",
            Self::DeleteFile => "CLIENT_SIDE_TOOL_V2_DELETE_FILE",
            Self::Reapply => "CLIENT_SIDE_TOOL_V2_REAPPLY",
            Self::GetRelatedFiles => "CLIENT_SIDE_TOOL_V2_GET_RELATED_FILES",
            Self::ParallelApply => "CLIENT_SIDE_TOOL_V2_PARALLEL_APPLY",
            Self::RunTerminalCommandV2 => "CLIENT_SIDE_TOOL_V2_RUN_TERMINAL_COMMAND_V2",
            Self::FetchRules => "CLIENT_SIDE_TOOL_V2_FETCH_RULES",
            Self::Planner => "CLIENT_SIDE_TOOL_V2_PLANNER",
            Self::WebSearch => "CLIENT_SIDE_TOOL_V2_WEB_SEARCH",
            Self::Mcp => "CLIENT_SIDE_TOOL_V2_MCP",
            Self::WebViewer => "CLIENT_SIDE_TOOL_V2_WEB_VIEWER",
            Self::DiffHistory => "CLIENT_SIDE_TOOL_V2_DIFF_HISTORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIENT_SIDE_TOOL_V2_UNSPECIFIED" => Some(Self::Unspecified),
            "CLIENT_SIDE_TOOL_V2_READ_SEMSEARCH_FILES" => Some(Self::ReadSemsearchFiles),
            "CLIENT_SIDE_TOOL_V2_READ_FILE_FOR_IMPORTS" => Some(Self::ReadFileForImports),
            "CLIENT_SIDE_TOOL_V2_RIPGREP_SEARCH" => Some(Self::RipgrepSearch),
            "CLIENT_SIDE_TOOL_V2_RUN_TERMINAL_COMMAND" => Some(Self::RunTerminalCommand),
            "CLIENT_SIDE_TOOL_V2_READ_FILE" => Some(Self::ReadFile),
            "CLIENT_SIDE_TOOL_V2_LIST_DIR" => Some(Self::ListDir),
            "CLIENT_SIDE_TOOL_V2_EDIT_FILE" => Some(Self::EditFile),
            "CLIENT_SIDE_TOOL_V2_FILE_SEARCH" => Some(Self::FileSearch),
            "CLIENT_SIDE_TOOL_V2_SEMANTIC_SEARCH_FULL" => Some(Self::SemanticSearchFull),
            "CLIENT_SIDE_TOOL_V2_CREATE_FILE" => Some(Self::CreateFile),
            "CLIENT_SIDE_TOOL_V2_DELETE_FILE" => Some(Self::DeleteFile),
            "CLIENT_SIDE_TOOL_V2_REAPPLY" => Some(Self::Reapply),
            "CLIENT_SIDE_TOOL_V2_GET_RELATED_FILES" => Some(Self::GetRelatedFiles),
            "CLIENT_SIDE_TOOL_V2_PARALLEL_APPLY" => Some(Self::ParallelApply),
            "CLIENT_SIDE_TOOL_V2_RUN_TERMINAL_COMMAND_V2" => {
                Some(Self::RunTerminalCommandV2)
            }
            "CLIENT_SIDE_TOOL_V2_FETCH_RULES" => Some(Self::FetchRules),
            "CLIENT_SIDE_TOOL_V2_PLANNER" => Some(Self::Planner),
            "CLIENT_SIDE_TOOL_V2_WEB_SEARCH" => Some(Self::WebSearch),
            "CLIENT_SIDE_TOOL_V2_MCP" => Some(Self::Mcp),
            "CLIENT_SIDE_TOOL_V2_WEB_VIEWER" => Some(Self::WebViewer),
            "CLIENT_SIDE_TOOL_V2_DIFF_HISTORY" => Some(Self::DiffHistory),
            _ => None,
        }
    }
}
/// aiserver.v1.RunTerminalCommandEndedReason
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunTerminalCommandEndedReason {
    Unspecified = 0,
    ExecutionCompleted = 1,
    ExecutionAborted = 2,
    ExecutionFailed = 3,
    ErrorOccurredCheckingReason = 4,
}
impl RunTerminalCommandEndedReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "RUN_TERMINAL_COMMAND_ENDED_REASON_UNSPECIFIED",
            Self::ExecutionCompleted => {
                "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_COMPLETED"
            }
            Self::ExecutionAborted => {
                "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_ABORTED"
            }
            Self::ExecutionFailed => "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_FAILED",
            Self::ErrorOccurredCheckingReason => {
                "RUN_TERMINAL_COMMAND_ENDED_REASON_ERROR_OCCURRED_CHECKING_REASON"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RUN_TERMINAL_COMMAND_ENDED_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_COMPLETED" => {
                Some(Self::ExecutionCompleted)
            }
            "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_ABORTED" => {
                Some(Self::ExecutionAborted)
            }
            "RUN_TERMINAL_COMMAND_ENDED_REASON_EXECUTION_FAILED" => {
                Some(Self::ExecutionFailed)
            }
            "RUN_TERMINAL_COMMAND_ENDED_REASON_ERROR_OCCURRED_CHECKING_REASON" => {
                Some(Self::ErrorOccurredCheckingReason)
            }
            _ => None,
        }
    }
}
/// aiserver.v1.ChunkType
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChunkType {
    Unspecified = 0,
    Codebase = 1,
    LongFile = 2,
    Docs = 3,
}
impl ChunkType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "CHUNK_TYPE_UNSPECIFIED",
            Self::Codebase => "CHUNK_TYPE_CODEBASE",
            Self::LongFile => "CHUNK_TYPE_LONG_FILE",
            Self::Docs => "CHUNK_TYPE_DOCS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHUNK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CHUNK_TYPE_CODEBASE" => Some(Self::Codebase),
            "CHUNK_TYPE_LONG_FILE" => Some(Self::LongFile),
            "CHUNK_TYPE_DOCS" => Some(Self::Docs),
            _ => None,
        }
    }
}
