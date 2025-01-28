//! This module handles tags and relations.

use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::constraints::Constraint;
use crate::objects::ObjectId;
use crate::types::PrimitiveType;

/// Represents a built-in tag by id. Built-in tags are always available.
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BuiltInTagId {
    LastRead = 32, // LastRead(DateTime)
    LastWritten = 33, // LastWritten(DateTime)
    Created = 34, // Created(DateTime)
    Sha256 = 35, // <inferred> Sha256(Binary)
    Owner = 36, // Owner(UserId): +File +Directory
    FileType = 37, // <inferred> File.Type: File.Type.Extension
    Source = 38, // Source(URL)
    Temporary = 39, // Temporary
    Entry = 40, // <multi> Entry(<property-based>)
    Author = 41, // Author(UserId)
    Size = 42, // <inferred> Size(Integer)
    Tag = 43,
    FileCount = 44, // <inferred> Directory.FileCount(Integer): +Directory
    DirectoryCount = 45,
    TotalFileCount = 46,
    TotalDirectoryCount = 47,
    Trashed = 48, // Trashed
    File = 49, // File(Binary)
    Directory = 50, // Directory: -File -User -Group -Tag
    Favourite = 51, // Favourite
    Image = 52, // <inferred> Image: +File
    ImageWidth = 53, // <inferred> Image.Width(Integer): +Image
    ImageHeight = 54,
    ImageBitDepth = 55,
    ImageCameraMaker = 56,
    ImageCameraModel = 57,
    ImageFStop = 58,
    ImageExposure = 59,
    ImageISO = 60,
    ImageFocalLength = 61,
    Title = 62, // Title(String)
    Path = 63, // <inferred> <array> Path(String): +Entry
    Text = 64, // <inferred> Text: +File
    TextWordCount = 65, // <inferred> Text.WordCount(Integer) +Text
    FileTypeExtension = 66, // File.Type.Extension(String) +File.Type.Inner
    FileTypeInner = 67, // File.Type.Inner(Object +File.Type)
    EntryName = 68, // Entry.Name(String)
    EntryParent = 69, // Entry.Parent(Object +Directory) +
    ReadAccess = 70, // ReadAccess(Object: User)
    WriteAccess = 71, // WriteAccess(Object: User)
    User = 72, // User: !File + !Directory + !Group + !Tag
    Group = 73, // Group -File -Directory -User -Tag
    TagSchema = 74, // Tag.Schema(Schema): Tag
    TagParent = 75, // Tag.Parent(optional Object: Tag): Tag
    References = 76,
    Language = 77,
    TagName = 78, // <array> Tag.Name(Object: Language + Title)
}

/*
impl BuiltInTagId {
    /// Returns whether `self` is inferred. This is known at compile time, because
    /// the enum defined built-in types.
    #[inline]
    pub const fn is_inferred(self) -> bool {
        match self {
            Self::LastRead => false,
            Self::LastWritten => false,
            Self::Created => false,
            Self::Sha256 => true,
            Self::Owner => false,
            Self::Type => false,
            Self::Source => false,
            Self::Temporary => false,
            Self::Entry => false,
            Self::Author => false,
            Self::Size => true,
            Self::Content => false,
            Self::FileCount => true,
            Self::DirectoryCount => true,
            Self::TotalFileCount => true,
            Self::TotalDirectoryCount => true,
            Self::Trashed => false,
            Self::File => true,
            Self::Directory => true,
            Self::Favourite => false,
            Self::Image => true,
            Self::ImageWidth => true,
            Self::ImageHeight => true,
            Self::ImageBitDepth => true,
            Self::ImageCameraMaker => true,
            Self::ImageCameraModel => true,
            Self::ImageFStop => true,
            Self::ImageExposure => true,
            Self::ImageISO => true,
            Self::ImageFocalLength => true,
            Self::Title => false,
            Self::Path => true,
            Self::Text => true,
            Self::TextWordCount => true,
            Self::TypeExtension => false,
            Self::TypeInner => false,
            Self::EntryName => false,
            Self::EntryParent => false,
            Self::ReadAccess => false,
            Self::WriteAccess => false,
        }
    }
}*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TagId {
    BuiltIn(BuiltInTagId),
    // UserDefined(UserDefinedTagId),
}

impl Into<i64> for TagId {
    fn into(self) -> i64 {
        match self {
            TagId::BuiltIn(id) => id as i64,
        }
    }
}

pub enum TagSchema<'a> {
    None,
    Primitive(PrimitiveType),
    OptionalPrimitive(Option<PrimitiveType>),
    ObjectReference(ObjectReference<'a>),
    OptionalObjectReference(Option<ObjectReference<'a>>),
}

pub struct ObjectReference<'a> {
    pub id: ObjectId,
    pub constraints: Option<Constraint<'a>>,
}