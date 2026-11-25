use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error("Instance not found")]
    NotFound,

    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("Mod not found")]
    ModNotFound,

    #[error("Instance is already running")]
    AlreadyStarted,

    #[error("Instance already exists")]
    AlreadyExists,

    #[error("Cannot modify an instance while it is running")]
    Busy,

    #[error("This instance already has an options.txt")]
    OptionsExists,

    #[error("A file operation is already in progress for this instance")]
    FileOperationInProgress,

    #[error("Log file not found")]
    LogNotFound,

    #[error("Error processing filename")]
    FilenameParse,

    #[error("Invalid instance name: {0}")]
    InstNameParse(String),

    #[error("Invalid source path")]
    InvalidSourcePath,

    #[error("Java {0} is not installed. Install it in Settings > Java.")]
    JreNotFound(String),

    #[error("Could not prepare minecraft.jar: {0}")]
    MinecraftJar(String),

    #[error("Unrecognized instance format in the provided file")]
    ImportInstanceFormatUnknown,

    #[error("Invalid or corrupted instance file: {0}")]
    ImportInstanceArchiveInvalid(String),

    #[error("Could not extract the instance archive: {0}")]
    ImportInstanceExtractionFailed(String),

    #[error("Could not determine the Minecraft version")]
    ImportInstanceUnknownMinecraftVersion,

    #[error("Invalid or expired import preview session")]
    ImportInstanceUnknownPreviewSession,

    #[error("Unsupported loader in instance: {0}")]
    ImportInstanceUnsupportedLoader(String),

    #[error("Import provider {provider} failed: {message}")]
    ImportInstanceProviderError { provider: String, message: String },

    #[error(transparent)]
    Fs(#[from] crate::core::errors::fs::FsError),
}

impl InstanceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "INST_NOT_FOUND",
            Self::VersionNotFound(_) => "INST_VERSION_NOT_FOUND",
            Self::ModNotFound => "INST_MOD_NOT_FOUND",
            Self::AlreadyStarted => "INST_ALREADY_STARTED",
            Self::AlreadyExists => "INST_EXISTS",
            Self::Busy => "INST_BUSY",
            Self::OptionsExists => "INST_OPTIONS_EXISTS",
            Self::FileOperationInProgress => "INST_FILES_BUSY",
            Self::LogNotFound => "INST_LOG_NOT_FOUND",
            Self::FilenameParse => "INST_FILENAME_PARSE",
            Self::InstNameParse(_) => "INST_NAME_INVALID",
            Self::InvalidSourcePath => "INST_SRC_INVALID",
            Self::JreNotFound(_) => "INST_JRE_MISSING",
            Self::MinecraftJar(_) => "INST_MINECRAFT_JAR",
            Self::ImportInstanceFormatUnknown => "INST_IMPORT_FORMAT_UNKNOWN",
            Self::ImportInstanceArchiveInvalid(_) => "INST_IMPORT_ARCHIVE_INVALID",
            Self::ImportInstanceExtractionFailed(_) => "INST_IMPORT_EXTRACTION_FAILED",
            Self::ImportInstanceUnknownMinecraftVersion => "INST_IMPORT_UNKNOWN_MINECRAFT_VERSION",
            Self::ImportInstanceUnknownPreviewSession => "INST_IMPORT_PREVIEW_EXPIRED",
            Self::ImportInstanceUnsupportedLoader(_) => "INST_IMPORT_UNSUPPORTED_LOADER",
            Self::ImportInstanceProviderError { .. } => "INST_IMPORT_PROVIDER_ERROR",
            Self::Fs(e) => e.code(),
        }
    }

    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::VersionNotFound(v) => vec![("version", v.clone())],
            Self::InstNameParse(s) => vec![("error", s.clone())],
            Self::JreNotFound(v) => vec![("version", v.clone())],
            Self::MinecraftJar(message) => vec![("error", message.clone())],
            Self::ImportInstanceArchiveInvalid(d) => vec![("error", d.clone())],
            Self::ImportInstanceExtractionFailed(d) => vec![("error", d.clone())],
            Self::ImportInstanceUnsupportedLoader(l) => vec![("loader", l.clone())],
            Self::ImportInstanceProviderError { provider, message } => {
                vec![("provider", provider.clone()), ("error", message.clone())]
            }
            Self::Fs(e) => e.params(),
            _ => vec![],
        }
    }
}
