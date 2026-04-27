use everything_structures::Abstract;

/// An extension to [Abstract], providing useful constants.
pub trait AbstractExt {
    const CONTAINS: Abstract = Abstract(1);
    const AXIOMATIC: Abstract = Abstract(2);
    const COMPUTED: Abstract = Abstract(3);
    const STATEMENT_SUBJECT: Abstract = Abstract(4);
    const STATEMENT_TAG: Abstract = Abstract(5);
    const STATEMENT_VALUE: Abstract = Abstract(6);
    const STATEMENT: Abstract = Abstract(7);
    const KNOWLEDGE: Abstract = Abstract(8);

    const NODE_LITERAL: Abstract = Abstract(12);
    const NODE_AND: Abstract = Abstract(13);
    const NODE_EXISTS: Abstract = Abstract(14);
    const NODE_PARAMETER: Abstract = Abstract(15);
    const NODE_COUNT: Abstract = Abstract(17);
    const NODE_QUERY: Abstract = Abstract(18);
    const NODE_EQUAL: Abstract = Abstract(19);
    const NODE_OR: Abstract = Abstract(20);
    const NODE_XOR: Abstract = Abstract(21);
    const NODE_NOT: Abstract = Abstract(22);
    // const NODE: Abstract = Abstract(23);
    // const TAG: Abstract = Abstract(24);
    const NODE_FUNCTION_SELF: Abstract = Abstract(25);
    const NODE_ADD_LEFT: Abstract = Abstract(2148566534810416742677953060919673357);
    const NODE_ADD_RIGHT: Abstract = Abstract(2148566534810416742677953060919673358);
}

impl AbstractExt for Abstract {}
