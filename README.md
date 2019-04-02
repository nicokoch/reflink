# reflink
[![](http://meritbadge.herokuapp.com/reflink)](https://crates.io/crates/reflink)
[![Build Status](https://travis-ci.org/nicokoch/reflink.svg?branch=master)](https://travis-ci.org/nicokoch/reflink)

Cross-platform(!) COW reflink copy of files

Some file systems implement COW (copy on write) functionality in order to speed up file copies.
On a high level, the new file does not actually get copied, but shares the same on-disk data with the source file.
As soon as one of the files is modified, the actual copying is done by the underlying OS.

This library supports Linux, Android, OSX, ios and Windows. As soon as other OS support the functionality, support will be added.
For implementation details, visit the [docs](https://docs.rs/reflink/*/reflink/).
