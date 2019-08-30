# Libraries for [CITA](https://github.com/cryptape/cita)

[![CircleCI](https://circleci.com/gh/cryptape/cita-common/tree/develop.svg?style=svg)](https://circleci.com/gh/cryptape/cita-common/tree/develop)
[![codecov](https://codecov.io/gh/cryptape/cita-common/branch/develop/graph/badge.svg)](https://codecov.io/gh/cryptape/cita-common)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fcryptape%2Fcita-common.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fcryptape%2Fcita-common?ref=badge_shield)

This repository is a collection of crates used by microservices in [CITA](https://github.com/cryptape/cita).

Crates below are extracted from [Parity](https://github.com/paritytech/parity):

- ethcore-bloom-journal
- rlp
- rlp_derive
- util
- db

with following modifications:

- add UtilError::Snappy in util::error
- add some modules in util:
    - build_info.rs
    - init.rs
    - instrument.rs
- add get_value_proof and verify_value_proof in db::trie::triedb


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fcryptape%2Fcita-common.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fcryptape%2Fcita-common?ref=badge_large)