# Libraries for [CITA](https://github.com/cryptape/cita)

[![CircleCI](https://circleci.com/gh/cryptape/cita-common/tree/develop.svg?style=svg)](https://circleci.com/gh/cryptape/cita-common/tree/develop)
[![codecov](https://codecov.io/gh/cryptape/cita-common/branch/develop/graph/badge.svg)](https://codecov.io/gh/cryptape/cita-common)

This repository is a collection of crates used by microservices in [CITA](https://github.com/cryptape/cita).

Crates below are extracted from [Parity](https://github.com/paritytech/parity):

- bigint
- bloomable
- ethcore-bloom-journal
- ethcore-io
- ethkey
- rlp
- rlp_derive
- sha3
- util

with following modifications:

- impl serde for bigint
- add UtilError::Snappy in util::error
- add some modules in util:
    - build_info.rs
    - datapath.rs
    - hashable.rs
    - init.rs
    - instrument.rs
    - merklehash.rs
    - snappy.rs
- add util::avl.
