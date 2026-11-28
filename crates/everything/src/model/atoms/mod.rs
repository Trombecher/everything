use std::fmt::Debug;

#[cfg(test)]
mod tests;

#[allow(dead_code)] // it is not dead code, it is used in the macro (!)
const fn first_duplicate(slice: &[u128]) -> Option<u128> {
    let mut i = 0;

    while i < slice.len() {
        let mut j = i + 1;

        while j < slice.len() {
            if slice[i] == slice[j] {
                return Some(slice[i]);
            }

            j += 1;
        }

        i += 1;
    }

    None
}

#[allow(dead_code)] // it is not dead code, it is used in the macro (!)
const fn first_duplicate_str<'a>(slice: &[&'a str]) -> Option<&'a str> {
    let mut i = 0;

    while i < slice.len() {
        let mut j = i + 1;

        while j < slice.len() {
            if str_eq(slice[i], slice[j]) {
                return Some(slice[i]);
            }

            j += 1;
        }

        i += 1;
    }

    None
}

const fn str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();

    if left.len() != right.len() {
        return false;
    }

    let mut i = 0;

    while i < left.len() {
        if left[i] != right[i] {
            return false;
        }

        i += 1;
    }

    true
}

/// An atomic object. Indivisible. Uncomposed.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Atom(pub u128);

macro_rules! declare_known_atoms {
    ($($(#[doc = $doc:literal])* $NAME:ident = $name:literal = $value:literal;)*) => {
        $(
            $(#[doc = $doc])*
            pub const $NAME: Self = Self($value);
        )*

        /// Returns the name of this atom in the documentation, if it is a
        /// built-in atom.
        #[must_use]
        pub const fn known_name(self) -> Option<&'static str> {
            match self.0 {
                $($value => Some($name),)*
                _ => None,
            }
        }

        const _CHECK: () = const {
            let values: &[u128] = &[$($value,)*];

            if let Some(_) = first_duplicate(values) {
                panic!("duplicate ids");
            }
        };

        const _CHECK_NAMES: () = const {
            let names: &[&str] = &[$($name,)*];

            if let Some(_) = first_duplicate_str(names) {
                panic!("duplicate names");
            }
        };
    };
}

#[allow(clippy::unreadable_literal)]
impl Atom {
    declare_known_atoms!(
        /// The atom 0 (zero).
        ZERO = "zero" = 2148623498527692884679683682469014909;

        /// The atom that is a property constructor for the next integer.
        SUCCESSOR_OF = "successor_of" = 2148623541928107461725235547620677496;

        /// The atom that is a property constructor for the previous integer.
        PREDECESSOR_OF = "predecessor_of" = 2150448436013648618415526635010123333;

        /// The slot for the item value in an list.
        LIST_ITEM = "list_item" = 2148623561006058368074852138222840554;

        /// Denotes the rest of the list.
        LIST_TAIL = "list_tail" = 2148623576528286069034804554905263388;

        /// Denotes the value of the character.
        CODE_POINT = "code_point" = 2148623592788974381730175331142334217;

        /// Denotes the bit zero/off/no.
        BIT_OFF = "bit_off" = 2148623606609578531456976446246481504;

        /// Denotes the bit one/on/yes.
        BIT_ON = "bit_on" = 2148623648901638714578593280489697845;

        /// The bit slot with index 0. Also called the _least significant_ bit.
        BIT_SLOT_0 = "bit_slot_0" = 2148623657764931307024391951942698252;

        /// The bit slot with index 1.
        BIT_SLOT_1 = "bit_slot_1" = 2148623779289647569021414352435474390;

        /// The bit slot with index 2.
        BIT_SLOT_2 = "bit_slot_2" = 2148623793297509473303838197251962004;

        /// The bit slot with index 3.
        BIT_SLOT_3 = "bit_slot_3" = 2148623803164736660711697141897226232;

        /// The bit slot with index 4.
        BIT_SLOT_4 = "bit_slot_4" = 2148623819303808917979289470225598602;

        /// The bit slot with index 5.
        BIT_SLOT_5 = "bit_slot_5" = 2148623826522742854507738117057019657;

        /// The bit slot with index 6.
        BIT_SLOT_6 = "bit_slot_6" = 2148623834321554672547365012703956257;

        /// The bit slot with index 7. Also called the _most significant_ bit.
        BIT_SLOT_7 = "bit_slot_7" = 2148623841265101367555509279336886325;

        NON_CONSTRUCTED_PROPERTY = "non_constructed_property" = 2166698849805927385476626148502402988;
        PROPERTY_CONSTRUCTOR = "property_constructor" = 2166698849805466203865788555426578088;

        NODE_FUNCTION = "node_function" = 2148623901005465698003044719488417081;

        NODE_LITERAL = "node_literal" = 2148623946948209931514052368378168923;

        NODE_AND_FIRST = "node_and_first" = 2148623952614130968570357528352754483;
        NODE_AND_SECOND = "node_and_second" = 2150546484857217536175129940420364334;

        NODE_PARAMETER = "node_parameter" = 2148623964016728126166347458070520202;

        NODE_SET_COUNT = "node_set_count" = 2148623971839749022702961541901456532;

        NODE_PROPERTIES_OF = "node_properties_of" = 2169866792562131836262938477060325203;

        NODE_HAS_A_SET_ITEM = "node_has_a_set_item" = 2169868497815503720673833180838489269;

        NODE_HAS_PROPERTY_SUBJECT = "node_has_property_subject" = 2170813895193454560979563081077252333;
        NODE_HAS_PROPERTY_PROPERTY = "node_has_property_property" = 2170813895193711757727776628945663643;

        NODE_EQUAL_FIRST = "node_equal_first" = 2148623984105467336671475554302291443;
        NODE_EQUAL_SECOND = "node_equal_second" = 2150546540588687321716707989954282134;

        NODE_OR_FIRST = "node_or_first" = 2148623991617605004082324671379584752;
        NODE_OR_SECOND = "node_or_second" = 2150546471301176810351954476414010805;

        NODE_XOR_FIRST = "node_xor_first" = 2148623998771243099015849175301884477;
        NODE_XOR_SECOND = "node_xor_second" = 2150546556818362149014272809860190195;

        NODE_NOT = "node_not" = 2148624004488096373987985726200830300;

        NODE_FUNCTION_SELF = "node_function_self" = 2148624013653182105587583289980365332;

        NODE_ADD_FIRST = "node_add_first" = 2148566534810416742677953060919673357;
        NODE_ADD_SECOND = "node_add_second" = 2148566534810416742677953060919673358;

        NODE_SET_UNION_FIRST = "node_set_union_first" = 2150602665196451232518504136866676495;
        NODE_SET_UNION_SECOND = "node_set_union_second" = 2150602676477464422271577313769115263;

        NODE_SET_INTERSECTION_FIRST = "node_set_intersection_first" = 6105450065514020261664789753451078433;
        NODE_SET_INTERSECTION_SECOND = "node_set_intersection_second" = 294371887698077017665747245360098400935;

        NODE_SET_DIFFERENCE_FIRST = "node_set_difference_first" = 90127457068241635743380726879163284651;
        NODE_SET_DIFFERENCE_SECOND = "node_set_difference_second" = 316434225730040209152190202706977983551;

        NODE_SET_MAP_SET = "node_set_map_set" = 2150755705084816915741037497738372617;
        NODE_SET_MAP_MAPPER = "node_set_map_mapper" = 2150755713297709286586583523030939076;

        NODE_SET_EVERY_SET = "node_set_every_set" = 2166698849805704507867628297969608633;
        NODE_SET_EVERY_PREDICATE = "node_set_every_predicate" = 2166698849806407199650439702292185631;

        NODE_SET_ANY_SET = "node_set_any_set" = 2166698849805453912896075884999506648;
        NODE_SET_ANY_PREDICATE = "node_set_any_predicate" = 2166698849805816631718884425726665751;

        NODE_SET_FILTER_SET = "node_set_filter_set" = 2150755714608308122129968313720999578;
        NODE_SET_FILTER_FILTER = "node_set_filter_filter" = 2150755715328094057860222292607728283;

        NODE_LESS_FIRST = "node_less_first" = 2150755802916608365774567517427204904;
        NODE_LESS_SECOND = "node_less_second" = 2150755809462832010701281576784730955;

        NODE_IF_CONDITION = "node_if_condition" = 2150756911395278548780220055702389669;
        NODE_IF_THEN = "node_if_then" = 2150756911395360151235546653103149781;
        NODE_IF_ELSE = "node_if_else" = 2150756911395636855407117245584275797;

        NODE_SET_UNWRAP_SET = "node_set_unwrap_set" = 2150860435707265317782695126754608091;

        NODE_MULTIPLY_FIRST = "node_multiply_first" = 2150955291898078111384483788783842606;
        NODE_MULTIPLY_SECOND = "node_multiply_second" = 2150955291897679523990373018161137292;

        NODE_CALL_CALLEE = "node_call_callee" = 2150967257692765401288058191156339282;
        NODE_CALL_WITH = "node_call_with" = 2150967257692538503396115970645049110;

        /// May be the result of a computation that received ill-formed
        /// objects as input. It is desireable to proove that expressions
        /// do not evaluate to this atom for every input.
        UNDEFINED = "undefined" = 2150546603789477801977750950892561896;

        NODE_CONSTRUCTION_VALUES_OBJECT = "node_construction_values_object" = 2166943953076489335344956167016992222;
        NODE_CONSTRUCTION_VALUES_PROPERTY_CONSTRUCTOR = "node_construction_values_property_constructor" = 2166943953075944059533195652762751502;

        NODE_ATOMS_WITH_PROPERTY = "node_atoms_with_property" = 2166943953075978973796333008418190812;

        NODE_SET_CONTAINS_ELEMENT = "node_set_contains_element" = 2166943953075778840020869770686077619;
        NODE_SET_CONTAINS_SET = "node_set_contains_set" = 2166943953076260545247311619503999088;

        NODE_IS_ATOM = "node_is_atom" = 2166698913622186896712941547852179611;
        NODE_IS_CONSTRUCTION = "node_is_construction" = 2166943953076020814661790508015684296;
        NODE_IS_COMPOSITE = "node_is_composite" = 2166944102907011869965838540132997515;
        NODE_IS_NODE = "node_is_node" = 2171028566323798747218439132407824159;

        NODE_NEW_CONSTRUCTION_CONSTRUCTOR = "node_new_construction_constructor" = 2166944102907028846001820613799425787;
        NODE_NEW_CONSTRUCTION_VALUE = "node_new_construction_value" = 2166944102906836737095248839738218859;

        NODE_NEW_SET_ITEM = "node_new_set_item" = 199021041172814012682334153578244309615;
        NODE_NEW_COMPOSITE_PROPERTY = "node_new_composite_property" = 2170872929671533249185501082459026776;

        NODE_CONSTRUCTOR_OF_CONSTRUCTION = "node_constructor_of_construction" = 2166944102906855387534895944602332938;
        NODE_VALUE_OF_CONSTRUCTION = "node_value_of_construction" = 2166944102906650686418158228301455892;

        NODE_SET_FOLD_SET = "node_set_fold_set" = 2166944102906103538111212580396221266;
        NODE_SET_FOLD_FOLDER = "node_set_fold_folder" = 2166944102906948913400158817641233982;
        NODE_SET_FOLD_INITIAL = "node_set_fold_initial" = 2166944102906490208705196484796567529;

        NODE_QUERY_CONSTRUCTIONS = "node_query_constructions" = 2169938683744888441848469861984944275;
        NODE_ATOMS_WITH_CONSTRUCTOR = "node_atoms_with_constructor" = 2169941030578384052544017889355444292;

        /// The property constructor for the atom of a pair.
        ATOM = "atom" = 2169938654927554609243319010096537148;

        /// The property constructor for the value of a pair.
        VALUE = "value" = 2169938676104045712632187346901831731;

        /// Literal truth.
        TRUE = "true" = 2166944102906490208705196484796567530;

        /// Literal falsehood.
        FALSE = "false" = 2170291666543565065425670792797371702;

        /// Set membership.
        CONTAINS = "contains" = 2170709517673194230291133647353683043;

        /// Numerator in a fraction.
        NUMERATOR = "numerator" = 2170709897728466098635434080350968542;

        /// Denominator in a fraction.
        DENOMINATOR = "denominator" = 2170709907418312990524200060941600479;
    );
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.known_name() {
            Some(name) => f.write_str(name),
            None => write!(f, "@{}", self.0),
        }
    }
}

impl From<bool> for Atom {
    fn from(value: bool) -> Self {
        if value { Self::TRUE } else { Self::FALSE }
    }
}

impl TryFrom<Atom> for bool {
    type Error = ();

    fn try_from(atom: Atom) -> Result<Self, Self::Error> {
        match atom {
            Atom::TRUE => Ok(true),
            Atom::FALSE => Ok(false),
            _ => Err(()),
        }
    }
}
