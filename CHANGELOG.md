TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.3 (2021-11-09)
=====

* removes VarCursor. so slow.
* adds read_inplace() into buf.rs for the fast.
* fix some bugs.
* adds minimum support rustc version into README.

0.1.2 (2021-11-04)
=====

* adds node cache for read and write
* changes file header signature: `siamdb`
* removes unused enum KeyType from src/filedb/mod.rs
* adds RecordSizeStats into src/filedb/mod.rs
* adds read_one_byte() and read_exact_small() into BufFile for the fast.
* fix: seek over the end.

0.1.1 (2021-10-30)
=====

* changes package name: shamdb to siamesedb.
* adds sementic types.
* changes the node size and the record size are variables.
* adds vu64.rs and vu64_io.rs.
* adds inner/dbxxx.rs.
* writes a lot of code but it's still incomplete.

0.1.0 (2021-09-23)
=====

first commit
