use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::objects::ObjectID;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Schema {
    PropertyBased = 0,
    Primitive(Primitive),
    BuiltIn(BuiltInTypeID),
    UserDefined(UserDefinedType)
}

impl Schema {
    #[inline]
    pub fn from_packed(packed_schema: i64) -> Option<Self> {
        match packed_schema {
            0 => Some(Self::PropertyBased),
            x if let Ok(primitive) = x.try_into() => Some(Self::Primitive(primitive)),
            x if let Ok(built_in) = x.try_into() => Some(Self::BuiltIn(built_in)),
            x if let Ok(ud) = x.try_into() => Some(Self::UserDefined(ud)),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum Primitive {
    Object = 1,
    Decimal = 2,
    Integer = 3,
    String = 4,
    Duration = 5,
    DateTime = 6,
    Boolean = 7,
    Character = 8,
    URL = 9,
    Binary = 10,
    Color = 11,
    Email = 12,
    User = 13,
    Group = 14
}

/// An object id that is supposed to be a type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserDefinedType(pub ObjectID);

impl TryFrom<i64> for UserDefinedType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
pub enum BuiltInTypeID {
    LastRead = 32,
    LastWritten = 33,
    Created = 34,
    Sha256 = 35,
    Owner = 36,
    Type = 37,
    Source = 38,
    Temporary = 39,
    Entry = 40,
    Author = 41,
    Size = 42,
    Content = 43,
    FileCount = 44,
    DirectoryCount = 45,
    TotalFileCount = 46,
    TotalDirectoryCount = 47,
    Trashed = 48,
    File = 49,
    Directory = 50,
    Favourite = 51,
    Image = 52,
    ImageWidth = 53,
    ImageHeight = 54,
    ImageBitDepth = 55,
    ImageCameraMaker = 56,
    ImageCameraModel = 57,
    ImageFStop = 58,
    ImageExposure = 59,
    ImageISO = 60,
    ImageFocalLength = 61,
    Title = 62,
    Path = 63,
    Text = 64,
    TextWordCount = 65,
    TypeExtension = 66,
    TypeInner = 67,
    EntryName = 68,
    EntryParent = 69,
    ReadAccess = 70,
    WriteAccess = 71,
}

impl BuiltInTypeID {
    /// Returns whether `self` is inferred. This is known at compile time, because
    /// the enum defined built-in types.
    pub const fn is_inferred(self) -> bool {
        match self {
            BuiltInTypeID::LastRead => false,
            BuiltInTypeID::LastWritten => false,
            BuiltInTypeID::Created => false,
            BuiltInTypeID::Sha256 => true,
            BuiltInTypeID::Owner => false,
            BuiltInTypeID::Type => false,
            BuiltInTypeID::Source => false,
            BuiltInTypeID::Temporary => false,
            BuiltInTypeID::Entry => false,
            BuiltInTypeID::Author => false,
            BuiltInTypeID::Size => true,
            BuiltInTypeID::Content => false,
            BuiltInTypeID::FileCount => true,
            BuiltInTypeID::DirectoryCount => true,
            BuiltInTypeID::TotalFileCount => true,
            BuiltInTypeID::TotalDirectoryCount => true,
            BuiltInTypeID::Trashed => false,
            BuiltInTypeID::File => true,
            BuiltInTypeID::Directory => true,
            BuiltInTypeID::Favourite => false,
            BuiltInTypeID::Image => true,
            BuiltInTypeID::ImageWidth => true,
            BuiltInTypeID::ImageHeight => true,
            BuiltInTypeID::ImageBitDepth => true,
            BuiltInTypeID::ImageCameraMaker => true,
            BuiltInTypeID::ImageCameraModel => true,
            BuiltInTypeID::ImageFStop => true,
            BuiltInTypeID::ImageExposure => true,
            BuiltInTypeID::ImageISO => true,
            BuiltInTypeID::ImageFocalLength => true,
            BuiltInTypeID::Title => false,
            BuiltInTypeID::Path => true,
            BuiltInTypeID::Text => true,
            BuiltInTypeID::TextWordCount => true,
            BuiltInTypeID::TypeExtension => false,
            BuiltInTypeID::TypeInner => false,
            BuiltInTypeID::EntryName => false,
            BuiltInTypeID::EntryParent => false,
            BuiltInTypeID::ReadAccess => false,
            BuiltInTypeID::WriteAccess => false,
        }
    }
}

/// A type ID that is not confirmed to be existing.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TypeID {
    BuiltIn(BuiltInTypeID),
    UserDefined(UserDefinedType),
}

impl TryFrom<i64> for TypeID {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if let Ok(built_in) = x.try_into() => Ok(Self::BuiltIn(built_in)),
            x if let Ok(ud) = x.try_into() => Ok(Self::UserDefined(ud)),
            _ => Err(())
        }
    }
}