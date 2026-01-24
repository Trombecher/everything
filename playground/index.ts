import {
	Database,
	formatReason,
	M_INFERRED,
	M_OBJECT,
	M_REQUIRES,
	M_REQUIRES_NOT,
	M_TAG,
	M_UNIQUE,
} from "./db";

const result = Database.tryFrom([
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

	[M_OBJECT, M_TAG, M_OBJECT],
	[M_OBJECT, M_INFERRED, 0],
]);

if (result instanceof Database) {
	const db = result;

	console.log(db.firstValue(42390482, M_OBJECT));
} else {
	console.log(formatReason(result));
}
