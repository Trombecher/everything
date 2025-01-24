use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;
use crate::objects::{DirectoryID, FileID, ObjectID};

/// The inferred associations cache.
///
/// Some types are dynamically inferred from other types. This infer
pub struct IAC {
    /// The Sha-256 hash of the content type. Needs to be invalidated when updating content.
    pub sha256: Mutex<HashMap<ObjectID, [u8; 32]>>,

    /// Size is defined for files, directories and tags.
    /// But this cache does not store file size, only directory and tag size.
    pub size: Mutex<HashMap<ObjectID, u64>>,

    /// File count of a directory. Needs to be invalidated when adding or removing
    /// direct children of this directory.
    pub file_count: Mutex<HashMap<DirectoryID, u64>>,

    /// Needs to be invalidated when adding or removing direct directory children of this directory.
    pub directory_count: Mutex<HashMap<DirectoryID, u64>>,

    /// Needs to be invalidated when adding or removing files from the subtree of this directory.
    pub total_file_count: Mutex<HashMap<DirectoryID, u64>>,

    /// Needs to be invalidated when adding or removing directories from the subtree of this directory.
    pub total_directory_count: Mutex<HashMap<DirectoryID, u64>>,

    pub directory_last_read: Mutex<HashMap<DirectoryID, i64>>,
    pub directory_last_written: Mutex<HashMap<DirectoryID, i64>>,

    /// Image association. Needs to be invalided when updating the extension of the file.
    pub image: Mutex<HashSet<FileID>>,

    pub image_width: Mutex<HashMap<FileID, u64>>,
    pub image_height: Mutex<HashMap<FileID, u64>>,
    pub image_camera_maker: Mutex<HashMap<FileID, u64>>,
    pub image_camera_model: Mutex<HashMap<FileID, u64>>,
    pub image_f_stop: Mutex<HashMap<FileID, u64>>,
    pub image_exposure: Mutex<HashMap<FileID, u64>>,
    pub image_iso: Mutex<HashMap<FileID, u64>>,
    pub image_focal_length: Mutex<HashMap<FileID, u64>>,

    pub text: Mutex<HashSet<FileID>>,
    pub text_word_count: Mutex<HashMap<FileID, u64>>,
}

impl IAC {
    pub async fn invalidate_image(&self, file_id: FileID) {
        self.image.lock().await.remove(&file_id);
        self.image_width.lock().await.remove(&file_id);
        self.image_height.lock().await.remove(&file_id);
        self.image_camera_maker.lock().await.remove(&file_id);
        self.image_camera_model.lock().await.remove(&file_id);
        self.image_f_stop.lock().await.remove(&file_id);
        self.image_exposure.lock().await.remove(&file_id);
        self.image_iso.lock().await.remove(&file_id);
        self.image_focal_length.lock().await.remove(&file_id);

        // TODO: tokio join! optimization (?)
    }
}