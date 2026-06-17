//! Module handling abstract objects via [Abstract].

/// An (ideally) globally unique abstract object identifier.
/// **This integer should not have any semantic meaning.
/// Please use ULIDs or some other generator that is a
/// combination of randomness and time.**
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Abstract(pub u128);

impl Abstract {
    /// The abstract object 0 (zero).
    pub const ZERO: Self = Self(2148623498527692884679683682469014909);

    /// Denotes that the current object is a successor of some integer.
    pub const SUCCESSOR_OF: Self = Self(2148623541928107461725235547620677496);

    /// Denotes that the current object is a predecessor of some integer.
    pub const PREDECESSOR_OF: Self = Self(2150448436013648618415526635010123333);

    /// The slot for the item value in an list.
    pub const LIST_ITEM: Self = Self(2148623561006058368074852138222840554);

    /// Denotes the rest of the list.
    pub const LIST_TAIL: Self = Self(2148623576528286069034804554905263388);

    /// Denotes the value of the character.
    pub const CODE_POINT: Self = Self(2148623592788974381730175331142334217);

    /// Denotes the bit zero/off/no.
    pub const BIT_0: Self = Self(2148623606609578531456976446246481504);

    /// Denotes the bit one/on/yes.
    pub const BIT_1: Self = Self(2148623648901638714578593280489697845);

    /// The bit slot with index 0. Also called the _least significant_ bit.
    pub const BIT_SLOT_0: Self = Self(2148623657764931307024391951942698252);

    /// The bit slot with index 1.
    pub const BIT_SLOT_1: Self = Self(2148623779289647569021414352435474390);

    /// The bit slot with index 2.
    pub const BIT_SLOT_2: Self = Self(2148623793297509473303838197251962004);

    /// The bit slot with index 3.
    pub const BIT_SLOT_3: Self = Self(2148623803164736660711697141897226232);

    /// The bit slot with index 4.
    pub const BIT_SLOT_4: Self = Self(2148623819303808917979289470225598602);

    /// The bit slot with index 5.
    pub const BIT_SLOT_5: Self = Self(2148623826522742854507738117057019657);

    /// The bit slot with index 6.
    pub const BIT_SLOT_6: Self = Self(2148623834321554672547365012703956257);

    /// The bit slot with index 7. Also called the _most significant_ bit.
    pub const BIT_SLOT_7: Self = Self(2148623841265101367555509279336886325);
}

impl std::fmt::Debug for Abstract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
