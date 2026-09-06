use everything_objects::Abstract;

/// An extension to [`Abstract`], providing useful constants.
pub trait AbstractExt {
    /// Denotes that the subject is a _set_ that contains the associated value.
    const CONTAINS: Abstract = Abstract(2148623885993955829985846061169569945);

    /// Denotes that the subject `T` (which [`AbstractExt::AXIOMATIC`] is used on)
    /// can be used as a tag stating truth.
    /// The value will be called (via [`crate::ext::ObjectExt::call`])
    /// on the subject `S` and then on the value `V` of each association of the form
    /// `(S, T, V)`.
    const AXIOMATIC: Abstract = Abstract(2148623894085078740305997247889559475);

    /// Denotes the function body of a function.
    const FUNCTION: Abstract = Abstract(2148623901005465698003044719488417081);

    /// The _subject_ slot of a statement.
    const STATEMENT_SUBJECT: Abstract = Abstract(2148623909053123893672709737372288428);

    /// The _tag_ slot of a statement.
    const STATEMENT_TAG: Abstract = Abstract(2148623916651203732644414190007253763);

    /// The _value_ slot of a statement.
    const STATEMENT_VALUE: Abstract = Abstract(2148623924076051576854508924514462036);

    /// A function that checks if the input object is knowledge.
    const KNOWLEDGE: Abstract = Abstract(2148623940012028447614031237706438049);

    const NODE_LITERAL: Abstract = Abstract(2148623946948209931514052368378168923);
    const NODE_AND_LEFT: Abstract = Abstract(2148623952614130968570357528352754483);
    const NODE_AND_RIGHT: Abstract = Abstract(2150546484857217536175129940420364334);
    const NODE_PARAMETER: Abstract = Abstract(2148623964016728126166347458070520202);
    const NODE_COUNT: Abstract = Abstract(2148623971839749022702961541901456532);
    const NODE_QUERY: Abstract = Abstract(2148623977746529761395662089576479852);
    const NODE_EQUAL_LEFT: Abstract = Abstract(2148623984105467336671475554302291443);
    const NODE_EQUAL_RIGHT: Abstract = Abstract(2150546540588687321716707989954282134);
    const NODE_OR_LEFT: Abstract = Abstract(2148623991617605004082324671379584752);
    const NODE_OR_RIGHT: Abstract = Abstract(2150546471301176810351954476414010805);
    const NODE_XOR_LEFT: Abstract = Abstract(2148623998771243099015849175301884477);
    const NODE_XOR_RIGHT: Abstract = Abstract(2150546556818362149014272809860190195);
    const NODE_NOT: Abstract = Abstract(2148624004488096373987985726200830300);
    const NODE_FUNCTION_SELF: Abstract = Abstract(2148624013653182105587583289980365332);
    const NODE_ADD_LEFT: Abstract = Abstract(2148566534810416742677953060919673357);
    const NODE_ADD_RIGHT: Abstract = Abstract(2148566534810416742677953060919673358);
    const NODE_UNION_LEFT: Abstract = Abstract(2150602665196451232518504136866676495);
    const NODE_UNION_RIGHT: Abstract = Abstract(2150602676477464422271577313769115263);
    const NODE_MAP_SET: Abstract = Abstract(2150755705084816915741037497738372617);
    const NODE_MAP_MAPPER: Abstract = Abstract(2150755713297709286586583523030939076);
    const NODE_FILTER_SET: Abstract = Abstract(2150755714608308122129968313720999578);
    const NODE_FILTER_FILTER: Abstract = Abstract(2150755715328094057860222292607728283);
    const NODE_LESS_LEFT: Abstract = Abstract(2150755802916608365774567517427204904);
    const NODE_LESS_RIGHT: Abstract = Abstract(2150755809462832010701281576784730955);
    const NODE_IF_CONDITION: Abstract = Abstract(2150756911395278548780220055702389669);
    const NODE_IF_THEN: Abstract = Abstract(2150756911395360151235546653103149781);
    const NODE_IF_ELSE: Abstract = Abstract(2150756911395636855407117245584275797);
    const NODE_UNWRAP_OR_SET: Abstract = Abstract(2150860435707265317782695126754608091);
    const NODE_UNWRAP_OR_DEFAULT: Abstract = Abstract(2150860435707466294816983480346985929);
    const NODE_MULTIPLY_LEFT: Abstract = Abstract(2150955291898078111384483788783842606);
    const NODE_MULTIPLY_RIGHT: Abstract = Abstract(2150955291897679523990373018161137292);
    const NODE_CALL_CALLEE: Abstract = Abstract(2150967257692765401288058191156339282);
    const NODE_CALL_WITH: Abstract = Abstract(2150967257692538503396115970645049110);

    /// A node that resolves to the knowledge structure.
    const NODE_KNOWLEDGE: Abstract = Abstract(2151276264541937640181087051018499973);

    /// Denotes that a computation arithmetically overflowed.
    const ARITHMETIC_OVERFLOW: Abstract = Abstract(2150546596946485525298114723305118383);

    /// Denotes that the result of a computation is undefined
    /// for the given inputs.
    const UNDEFINED: Abstract = Abstract(2150546603789477801977750950892561896);
}

impl AbstractExt for Abstract {}
