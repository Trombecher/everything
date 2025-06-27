# Validation In Everything

Optimizing for read-heavy workloads, Everything memory-maps the database file. But storing invariants may lead to problems. For example, a string (UTF-8) that was stored before shutting down the process may be modified illegally while the process is down. Thus, the process (and therefore Rust code) cannot _rely_ on the text being correct UTF-8. It has to re-verify that.

While the DB process is running, however, Everything disallows other processes to access the file, as this may lead to data corruption. Therefore, the process can _assume_ that it owns the whole file and rely on the data being valid after it has been verified. But how does everything know if the content was already validated during the running process?

## Validation Ids

Everytime Everything's process starts and reads the DB's meta-page, a counter is incremented. This is the **validation id**. Every page in the file has a validation id slot. The `u64` stored there is the validation id from the process that has last validated the page.

So every time a page is accessed (in bulk), the validation id is checked, and if it is not equal to the validation id of the current process, the whole page is validated and the slot updated.

This system provides safe data to the DB and is fast because only accessed pages are re-validated (at first time access).