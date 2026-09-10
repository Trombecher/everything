# Cache-based managed storage abstraction layer

- code, fetching pages and stuff
    - file / disk partition adapters
- page cache holding N pages
- page crc validation when untrusted content is loaded into cache
- page caching strategies
    - you should be able to hint to the cache that your page
      access is only temporary and should not be held in cache.
- page eviction strategies (free list)
- page table mapping each page id to a page cache slot index
- page access manager
    - which page is in use how many times and open with what interpretation
    - (mapping from slot index to slot info)

## Acquiring a reference to a page

Call `page(RawPageId: raw_page_id, use_as: PageKind)`.

1. Lookup cache slot index for `raw_page_id`.
2. If this page is not found in the page cache,
    - load it from disk into the cache (evict if cache is full),
    - validate the CRC32C, and
    - add the entry for the page slot in the page table.
