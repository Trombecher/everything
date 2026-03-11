#set document(title: "Everything: Universal Structures")

= Mathematical Description

== Abstract Objects

The model assumes a set of abstract objects $A$. It could be defined as the natural numbers $NN$, or any other set.

== Objects

The set of all objects is defined by $O := A union S$, the union of all abstract objects and all structures. The set $S$ will be defined shortly.

== Properties

Properties relate objects. The set of all properties is defined by $P := O times O$. Therefore, a property is a 2-tuple. The first entry can be interpreted as the _tag_ and the second entry can be interpreted as the _value_.

== Structures

Structures are objects that are defined and uniquely identified by their properties. The set of all structures is defined by:

$
  S := {U subset.eq P | |U| < infinity}
$

Therefore, structures are finite sets of properties.
