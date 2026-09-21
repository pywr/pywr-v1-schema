# Changelog

All notable changes to this project will be documented in this file.


## [0.25.0] - 2026-09-12

### 🚀 Features

- Implement FromStr and improve error handling. ([#294](https://github.com/pywr/pywr-v1-schema/pull/294))


## [0.24.0] - 2026-08-07

### 🐛 Bug Fixes

- Fix some clippy warnings.
- Fix one more clippy lint.

### Chore

- *(deps)* [**breaking**] Switch from chrono to jiff for datetime support. ([#288](https://github.com/pywr/pywr-v1-schema/pull/288))


## [0.23.0] - 2026-07-07

### 🚀 Features

- Add user_combinations to model schema ([#282](https://github.com/pywr/pywr-schema/pull/282))


## [0.22.1] - 2026-06-02

### 🚀 Features

- Add meta method to Parameter ([#276](https://github.com/pywr/pywr-schema/pull/276))
- Add a method to retrieve a reference to NodeMeta ([#273](https://github.com/pywr/pywr-schema/pull/273))

### 🐛 Bug Fixes

- Retain tags when deserialising parameters ([#277](https://github.com/pywr/pywr-schema/pull/277))


## [0.22.0] - 2026-03-18

### 🚀 Features

- Add is_virtual method to CoreNode ([#270](https://github.com/pywr/pywr-schema/pull/270))
- Add tags to NodeMeta and ParameterMeta. ([#272](https://github.com/pywr/pywr-schema/pull/272))
- Add PywrMultiModel to the schema. ([#266](https://github.com/pywr/pywr-schema/pull/266))

### ⚙️ Miscellaneous Tasks

- Remove unneeded parentheses. ([#267](https://github.com/pywr/pywr-schema/pull/267))


## [0.21.2] - 2026-02-27

### 🐛 Bug Fixes

- Do not serialize name fields in parameters and tables. ([#264](https://github.com/pywr/pywr-schema/pull/264))


## [0.21.1] - 2026-02-07

### ⚙️ Miscellaneous Tasks

- Drop specific patch numbers in Cargo.toml ([#257](https://github.com/pywr/pywr-schema/pull/257))


## [0.21.0] - 2025-09-26

### 🚀 Features

- Add constant scenario parameter to schema ([#239](https://github.com/pywr/pywr-schema/pull/239))


## [0.20.0] - 2025-09-23

### 🐛 Bug Fixes

- Add missing table attributes to v1 schema ([#236](https://github.com/pywr/pywr-schema/pull/236))
- Add checksum attribute to dataframe parameter ([#237](https://github.com/pywr/pywr-schema/pull/237))
- Add slice attribute to scenario ([#238](https://github.com/pywr/pywr-schema/pull/238))


## [0.19.0] - 2025-08-22

### 🚀 Features

- Add additional predicate aliases (LT, GE, etc) ([#225](https://github.com/pywr/pywr-schema/pull/225))

### 🐛 Bug Fixes

- Remove incorrect predicate field on multiple threshold parameters. ([#224](https://github.com/pywr/pywr-schema/pull/224))
- Show elided lifetimes to reduce confusion. ([#226](https://github.com/pywr/pywr-schema/pull/226))

## [0.18.0] - 2025-06-10

### 🐛 Bug Fixes

- Add is_variable field to ParameterMeta. (#208)

## [0.17.0] - 2024-12-16

### 🚀 Features

- Add most remaining threshold parameters to schema (#153)

## [0.16.0] - 2024-11-05

### 🚀 Features

- Add OptionalParameterValues type ([#140](https://github.com/pywr/pywr-schema/pull/140))

## [0.15.3] - 2024-10-29

### 🚀 Features

- Add more deserialisation aliases to node enum ([#137](https://github.com/pywr/pywr-schema/pull/137))

## [0.15.0] - 2024-09-10

### 🚜 Refactor

- Move PywrSchemaError to the top-level module. ([#96](https://github.com/pywr/pywr-schema/pull/96))

## [0.14.0] - 2024-07-25

### 🐛 Bug Fixes

- Add missing cost field to RiverGaugeNode. (#92)

### 🚜 Refactor

- [**breaking**] Renamed crates to be pywr-v1-xxx (#90)

### ⚙️ Miscellaneous Tasks

- Setup Release Please and modernise CI. (#91)
- Fix Release Please config (6).
- Inherit package attributes from workspace.
- Configure CHANGELOG for each crate.

## [0.13.1] - 2024-07-15

### 🎨 Styling

- Correct Clippy lint regarding use of .clone() (#70)

## [0.13.0] - 2024-04-26

### 🚀 Features

- Separate out the network from the model. (#62)

### ⚙️ Miscellaneous Tasks

- Release v0.13.0

## [0.12.0] - 2024-04-23

### 🚜 Refactor

- Use chrono instead of time for consistency with pywr-next. (#60)

### ⚙️ Miscellaneous Tasks

- Release v0.12.0 (#61)

## [0.11.2] - 2024-04-19

### 🐛 Bug Fixes

- Remove parameter attribute from ControlCurveIndexParameter definition (#54)

### ⚙️ Miscellaneous Tasks

- Release v0.11.2 (#58)

## [0.11.1] - 2024-02-15

### ⚙️ Miscellaneous Tasks

- Update strum to v0.26 (#24)
- Add dependabot config. (#25)
- Release v0.11.1 (#33)

## [0.11.0] - 2024-01-03

### 🚀 Features

- Implemented NegativeMaxParameter and NegativeMinParameter (#22)

### ⚙️ Miscellaneous Tasks

- Release v0.11.0 (#23)

## [0.10.0] - 2023-12-12

### 🚀 Features

- Add strum EnumVariantNames macro to CoreNode enum (#20)

### ⚙️ Miscellaneous Tasks

- Release v0.10.0 (#21)

## [0.9.0] - 2023-11-25

### 🚀 Features

- Add RbfProfileParameter. (#16)

### 🐛 Bug Fixes

- Make RbfProfileParameter public in parameters module. (#18)

### ⚙️ Miscellaneous Tasks

- Release v0.9.0 (#19)

## [0.8.0] - 2023-10-13

### 🚀 Features

- Add additional parameters.
- Improve resource path discovery.
- Use proc-macro to generate some Node methods.
- Add PywrParameter derive

### 🐛 Bug Fixes

- Skip serializing None for most fields.

### ⚙️ Miscellaneous Tasks

- Add additional comments to pywr-schema-macros
- Release v0.8.0 (#15)

## [0.7.0] - 2023-09-16

### 🚀 Features

- Added MinParameter 
- Added DivisionParameter schema (#12)

### ⚙️ Miscellaneous Tasks

- Release v0.7.0 (#13)

## [0.6.0] - 2023-05-10

### 🚀 Features

- Derive Clone for the whole schema.
- Derive Clone for the whole schema. (#9)

### ⚙️ Miscellaneous Tasks

- Release v0.6.0 (#10)

## [0.5.3] - 2023-03-06

### 🐛 Bug Fixes

- Fix DelayNode schema. (#7)

### ⚙️ Miscellaneous Tasks

- Setup CI. (#8)
- Release v0.5.3

## [0.5.2] - 2023-03-06

### 🐛 Bug Fixes

- Add missing nsteps attribute of PiecewiseLink. (#6)

### ⚙️ Miscellaneous Tasks

- Release v0.5.2

## [0.5.1] - 2023-03-03

### 🐛 Bug Fixes

- Remove unused imports.
- River split node factors should not be optional.

### ⚙️ Miscellaneous Tasks

- Release v0.5.1

## [0.5.0] - 2023-03-03

### 🚀 Features

- Add interp_day to MonthlyProfileParameter. (#3)
- Add schemas for remaining nodes (#1)

### 🐛 Bug Fixes

- Add missing index_parameter to parameters() method in IndexedArrayParameter. (#2)

## [0.4.0] - 2023-01-31

### 🚀 Features

- Add methods for finding parameters.

### ⚙️ Miscellaneous Tasks

- Release v0.4.0

## [0.3.0] - 2023-01-20

### 🚀 Features

- Add additional tests and swap enum order in ParameterValue.

### ⚙️ Miscellaneous Tasks

- Release v0.3.0

## [0.2.0] - 2022-12-21

### 🚀 Features

- Add additional nodes and parameters.

<!-- generated by git-cliff -->
