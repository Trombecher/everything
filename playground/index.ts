import {
    Database,
    formatReason,
    M_INFERRED,
    M_OBJECT,
    M_REQUIRES,
    M_REQUIRES_NOT,
    M_REQUIRES_OR,
    M_REQUIRES_OR_NOT,
    M_TAG,
    M_UNIQUE,
    type ValidationResult,
} from "./db";

const db = Database.tryFrom([
    [M_TAG, M_TAG, M_TAG],
    [M_TAG, M_UNIQUE, M_TAG],

    [M_UNIQUE, M_TAG, M_OBJECT],
    [M_UNIQUE, M_REQUIRES, M_TAG],
    [M_UNIQUE, M_REQUIRES_NOT, M_INFERRED],

    [M_INFERRED, M_TAG, M_OBJECT],
    [M_INFERRED, M_REQUIRES, M_TAG],

    [M_REQUIRES, M_TAG, M_TAG],
    [M_REQUIRES, M_REQUIRES, M_TAG],

    [M_REQUIRES_NOT, M_TAG, M_TAG],
    [M_REQUIRES_NOT, M_REQUIRES, M_TAG],

    [M_REQUIRES_OR, M_TAG, M_TAG],
    [M_REQUIRES_OR, M_REQUIRES, M_TAG],

    [M_REQUIRES_OR_NOT, M_TAG, M_TAG],
    [M_REQUIRES_OR_NOT, M_REQUIRES, M_TAG],

    [M_OBJECT, M_TAG, M_OBJECT],
    [M_OBJECT, M_INFERRED, 0],
]);

const unwrap = (result: ValidationResult) => {
    if (result === true) return;

    console.error(result);
    process.exit(-1);
};

if (!(db instanceof Database)) {
    console.error(formatReason(db));
    process.exit(-1);
}

const PERSON = 1001;
const HUMAN = 1002;
const DAVID = 1003;

unwrap(
    db.modify([
        {type: "add", association: [HUMAN, M_TAG, M_OBJECT]},
        {type: "add", association: [DAVID, HUMAN, 0]},
        {type: "add", association: [PERSON, M_TAG, M_OBJECT]},
        {type: "add", association: [PERSON, M_INFERRED, 0]},
        {type: "add", association: [PERSON, M_REQUIRES, HUMAN]},
    ]),
);

console.log(db.firstValue(DAVID, PERSON));
