# Chapter 5 - Across Knowledges

The previous chapters described behaviour of objects in a single knowledge base. But as this project grows, so do knowledge bases, **independently**. Reasoning about knowledge of two or more knowledge bases (implicitly) unions them into one knowledge base. It is inevitable that two knowledges both try to model the same things but with different atoms, so to merge their statements the maintainers need to **rename** atom ids locally, so they match ids with the other one.

Even if all knowledge bases are valid on their own, this does not mean that the union is valid. Merging them may lead the maintainers to discover such cases. It is then the maintainer's job to change around constraints, so that the union is valid knowledge.
