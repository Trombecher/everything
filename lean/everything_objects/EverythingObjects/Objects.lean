abbrev UInt128 := Fin (2 ^ 128)

structure AbstractObject : Type where
  value : UInt128
  deriving Ord

mutual
  inductive RawObject : Type where
    | Abstract : AbstractObject -> RawObject
    | Property : RawProperty -> RawObject
    | Set : List RawObject -> RawObject
    deriving Ord

  structure RawProperty : Type where
    constructor : RawObject
    value : RawObject
    deriving Ord
end

mutual
  def RawObject.Valid : RawObject -> Prop
    | .Abstract _ => True
    | .Property p => p.Valid
    | .Set elements =>
        elements.Pairwise (fun a b => compare a b = .lt) ∧ AllObjectsAreValid elements

  def RawProperty.Valid : RawProperty -> Prop
    | ⟨constructor, value⟩ => constructor.Valid ∧ value.Valid

  def AllObjectsAreValid : List RawObject -> Prop
    | [] => True
    | x :: xs => x.Valid ∧ AllObjectsAreValid xs
end

structure Object : Type where
  object : RawObject
  Valid : object.Valid
  deriving Ord

structure Property : Type where
  property : RawProperty
  Valid : property.Valid
  deriving Ord

def object1 : Object := ⟨RawObject.Abstract ⟨1⟩, trivial⟩
def object2 : Object := ⟨RawObject.Property ⟨RawObject.Abstract ⟨1⟩, RawObject.Abstract ⟨2⟩⟩, ⟨trivial, trivial⟩⟩
