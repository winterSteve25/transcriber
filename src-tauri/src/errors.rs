use std::io;

use epub::doc::DocError;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    FailedToReadEpub(#[from] DocError),

    #[error(transparent)]
    ExtractError(#[from] zip_extract::ZipExtractError),
    
    #[error("Failed to resolve app cache directory")]
    FailedToResolveAppCacheDir,
    
    #[error("Failed to resolve app data directory")]
    FailedToResolveAppDataDir,
    
    #[error("Failed to resolve epub cover path")]
    FailedToResolveEpubCoverPath,
    
    #[error("Failed to get epub file name")]
    FailedToGetEpubName,

    #[error("Failed to get epub title")]
    FailedToGetEpubTitle,

    #[error("Failed to get epub cover")]
    FailedToGetEpubCover,

    #[error("Failed to get epub creator")]
    FailedToGetEpubCreator,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
