export class BTree<K extends number, V> {
    root?: BTreeNode<K, V>;

    insert(key: K, value: V): BTree<K, V> {
    }

    getValue(key: K): V | undefined {
        return this.root?.getValue(key);
    }
}

const ORDER = 5;

class BTreeNode<K extends number, V> {
    private readonly keys: readonly K[] = [];
    private readonly values: readonly V[] = [];
    private readonly children: BTreeNode<K, V>[] = [];



    getValue(key: K): V | undefined {
        let index = 0;

        while(true) {
            const currentKey = this.keys[index];

            if(currentKey === key) break;
            if(currentKey === undefined || currentKey > key) {
                index += ORDER - 1;
                break;
            }

            index++;
        }

        if(index >= ORDER - 1) {
            return this.children?.[index - (ORDER - 1)]!.getValue(key);
        }

        return this.values[index];
    }

    isLeaf() {
        return Array.isArray(this.childrenOrNext);
    }
}