# Paging

The database file starts with one page of metadata (4096 bytes) and then is followed up by _blocks_.

## Blocks

One block is 2^14 = 16,384 bytes. All free block "slots" are connected in a free list (for O(1) allocation time).
Metadata in the meta-page tracks the free list.