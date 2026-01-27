export const M_TAG = 1;
export const M_UNIQUE = 2;
export const M_INFERRED = 3;
export const M_REQUIRES = 4;
export const M_REQUIRES_NOT = 5;
export const M_REQUIRES_OR = 6;
export const M_REQUIRES_OR_NOT = 7;
export const M_OBJECT = 8;

export const builtInObjectMap: Readonly<Record<ObjectId, string>> = {
	[M_TAG]: "M_TAG",
	[M_UNIQUE]: "M_UNIQUE",
	[M_INFERRED]: "M_INFERRED",
	[M_REQUIRES]: "M_REQUIRES",
	[M_REQUIRES_NOT]: "M_REQUIRES_NOT",
	[M_REQUIRES_OR]: "M_REQUIRES_OR",
	[M_REQUIRES_OR_NOT]: "M_REQUIRES_OR_NOT",
	[M_OBJECT]: "M_OBJECT",
};

export const formatObjectId = (n: ObjectId) =>
	builtInObjectMap[n] ?? objectIdToString(n);

export type ObjectId = number;

export type Association = [object: ObjectId, tag: ObjectId, value: ObjectId];

export type Modification = {
	type: "add" | "remove";
	association: Readonly<Association>;
};

export const objectIdToString = (objectId: ObjectId): string => `${objectId}`;
export const objectIdFromString = (s: string): ObjectId => Number(s);

const unpackAssociation = (s: string): Association =>
	s.split(":").map(objectIdFromString) as Association;

const packAssociation = (a: Readonly<Association>) =>
	a.map(objectIdToString).join(":");

export type ValidationResult = true | Reason;

export type Reason = {
	type: "missing" | "found";
	object: ObjectId | "some";
	tag: ObjectId | "some";
	value: ObjectId | "some";
};

export const formatReason = ({
	type,
	object,
	tag,
	value,
}: Readonly<Reason>): string => {
	return `${type} association (${object === "some" ? "_" : formatObjectId(object)}, ${tag === "some" ? "_" : formatObjectId(tag)}, ${value === "some" ? "_" : formatObjectId(value)})`;
};

export class Database {
	private readonly associations: Set<string>;

	public static tryFrom(
		associations: Readonly<Association>[],
	): Database | Reason {
		const set = new Set(associations[Symbol.iterator]().map(packAssociation));

		const db = new Database(set);
		const result = db.validate();

		if (result !== true) return result;

		return db;
	}

	private constructor(associations: Set<string>) {
		this.associations = associations;
	}

	private validate(): ValidationResult {
		if (this.firstValue(M_TAG, M_UNIQUE) === undefined)
			return {
				type: "missing",
				object: M_TAG,
				tag: M_UNIQUE,
				value: "some",
			};

		// Constraint (1) -- tags and values
		for (const [object, tag, value] of this.iterStored()) {
			const valueTag = this.tagValueConstraint(tag);
			if (valueTag === undefined)
				return {
					type: "missing",
					object: tag,
					tag: M_TAG,
					value: "some",
				};

			const matchResult = this.match(object, tag);
			if (matchResult !== true) return matchResult;

			if (this.firstValue(value, valueTag) === undefined)
				return {
					type: "missing",
					object: value,
					tag: valueTag,
					value: "some",
				};
		}

		// Constraint (2) -- uniqueness
		for (const [tag, _] of this.iterStoredObjectsAndValues(M_UNIQUE)) {
			for (const object of this.iterStoredObjects(tag)) {
				const secondValue = this.iterValues(object, tag).drop(1).next().value;

				if (secondValue !== undefined)
					return {
						type: "found",
						object,
						tag,
						value: secondValue,
					};
			}
		}

		// Constraint (6) -- inferredness

		for (const [tag, _f] of this.iterStoredObjectsAndValues(M_INFERRED)) {
			const storedObjectAndValue =
				this.iterStoredObjectsAndValues(tag).next().value;

			if (storedObjectAndValue !== undefined)
				return {
					type: "found",
					object: storedObjectAndValue[0],
					tag,
					value: storedObjectAndValue[1],
				};
		}

		return true;
	}

	public modify(modifications: Modification[]) {
		// Remove redundant modifications:

		let index = 0;
		while (index < modifications.length) {
			// biome-ignore lint/style/noNonNullAssertion: index checked
			const { type, association } = modifications[index]!;

			if (
				(type === "add" && this.hasStored(association)) ||
				(type === "remove" && !this.hasStored(association))
			) {
				// Remove redundant modification
				modifications.splice(index, 1);
			} else {
				index++;
			}
		}

		// Apply changes
		for (const { type, association } of modifications) {
			if (type === "add") {
				this.unsafeAddAssociation(association);
			} else {
				this.unsafeRemoveAssociation(association);
			}
		}

		if (modifications.length === 0 || this.validate()) {
			// Changes are valid or there are no (real) changes.
			return;
		}

		// Changes are invalid -> revert changes
		for (const { type, association } of modifications) {
			if (type === "add") {
				this.unsafeRemoveAssociation(association);
			} else {
				this.unsafeAddAssociation(association);
			}
		}
	}

	public compute(f: ObjectId, _object: ObjectId) {
		// TODO: compute
		return f;
	}

	private unsafeAddAssociation(a: Readonly<Association>) {
		this.associations.add(packAssociation(a));
	}

	private unsafeRemoveAssociation(a: Readonly<Association>) {
		this.associations.delete(packAssociation(a));
	}

	private match(object: ObjectId, tag: ObjectId): ValidationResult {
		for (const requiredTag of this.iterStoredValues(tag, M_REQUIRES)) {
			if (this.firstValue(object, requiredTag) === undefined)
				return {
					type: "missing",
					object,
					tag: requiredTag,
					value: "some",
				};
		}

		for (const requiredNotTag of this.iterStoredValues(tag, M_REQUIRES_NOT)) {
			if (this.firstValue(object, requiredNotTag) !== undefined)
				return {
					type: "found",
					object,
					tag: requiredNotTag,
					value: "some",
				};
		}

		// TODO

		return true;
	}

	iterStored(): IteratorObject<Readonly<Association>, undefined, undefined> {
		return this.associations[Symbol.iterator]().map(unpackAssociation);
	}

	iterStoredObjects(
		tag: ObjectId,
	): IteratorObject<ObjectId, undefined, undefined> {
		const visitedObjects = new Set<ObjectId>();

		return this.iterStored()
			.filter(([o, t, _v]) => t === tag && !visitedObjects.has(o))
			.map(([o, _t, _v]) => {
				visitedObjects.add(o);
				return o;
			});
	}

	iterStoredObjectsAndValues(
		tag: ObjectId,
	): IteratorObject<[object: ObjectId, value: ObjectId], undefined, undefined> {
		return this.iterStored()
			.filter(([_o, t, _v]) => t === tag)
			.map(([o, _t, v]) => [o, v]);
	}

	iterStoredValues(
		object: ObjectId,
		tag: ObjectId,
	): IteratorObject<ObjectId, undefined, undefined> {
		return this.iterStored()
			.filter(([o, t, _v]) => o === object && t === tag)
			.map(([_o, _t, v]) => v);
	}

	iterValues(
		object: ObjectId,
		tag: ObjectId,
	): IteratorObject<ObjectId, undefined, undefined> {
		const f = this.tagInferredValue(tag);

		if (f === undefined) {
			// Tag is not inferred.
			return this.iterStoredValues(object, tag);
		}

		throw new Error("TODO");
	}

	firstStoredValue(object: ObjectId, tag: ObjectId) {
		return this.iterStoredValues(object, tag).next().value;
	}

	firstValue(object: ObjectId, tag: ObjectId) {
		const f = this.tagInferredValue(tag);

		if (f !== undefined) return this.compute(f, object);

		return this.firstStoredValue(object, tag);
	}

	hasStored(association: Readonly<Association>) {
		return this.associations.has(packAssociation(association));
	}

	private isTag(tag: ObjectId): boolean {
		return this.tagValueConstraint(tag) !== undefined;
	}

	private tagValueConstraint(tag: ObjectId): ObjectId | undefined {
		return this.firstStoredValue(tag, M_TAG);
	}

	private tagInferredValue(tag: ObjectId): ObjectId | undefined {
		return this.firstStoredValue(tag, M_INFERRED);
	}
}
