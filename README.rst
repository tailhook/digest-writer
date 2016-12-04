=============
Digest Writer
=============

:Status: Beta
:Documentation: http://docs.rs/digest-writer/


This crate adds ``io::Write`` interface for ``digest::Digest``. Which
means you can use ``io::copy``, ``write!`` macros and other abstractions
with digests.

Example:

.. code-block:: rust

    use std::fs::File;
    use std::io::{self, Write};
    use sha2::Sha256;
    use digest::Writer;

    let digest = Writer::new(Sha256::new());
    let mut f = File::open("write.rs").unwrap();
    io::copy(f, digest).unwrap();
    digest.result();


License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

