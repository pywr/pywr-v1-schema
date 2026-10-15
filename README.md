<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache-2 License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">Pywr-schema</h3>

  <p align="center">
    A schema and JSON validator for <a href="https://github.com/pywr/pywr">Pywr</a>. 
    <br />
    <br />    
    <a href="https://github.com/pywr/pywr-schema/issues">Report Bug</a>
    ·
    <a href="https://github.com/pywr/pywr-schema/issues">Request Feature</a>
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[Pywr](https://github.com/pywr/pywr) is a water resources modelling library. It uses a JSON file
format for model definition. Unfortunately it contains no strong schema definition and uses 
bespoke deserialization methods. This makes parsing model definitions in other tools difficult.

Several attempts have been made to add and strengthen the schema definition in Pywr itself, but
these have become difficult to implement due to backward compatibility reasons. Therefore, this
project has been created as a separate definition of the Pywr JSON schema. It is implemented in
Rust using `serde` and `serde_json`. It aims to achieve schema compatibility with the latest
version of Pywr while also supporting custom nodes and parameters. 

The package is split into a library, `pywr_schema`, and validator `pywr_validator`. Other tools
may make use of the library for parsing Pywr JSON. 


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Pywr-schema is built with Rust and requires a working
[installation](https://www.rust-lang.org/tools/install). To validate a Pywr JSON file run the
`pywr-v1-validator`'s `validate` command as follows. 


   ```sh
   git clone https://github.com/pywr/pywr-schema.git
   cd pywr-schema
   cargo r --bin pywr-v1-validator -- validate --path /path/to/my-model.json
   ```

### JSON Schema

The types in the library can also generate a [JSON Schema](https://json-schema.org/) (draft
2020-12) for Pywr v1 model files, for use by editors and other tools. Write it with the
validator's `export-schema` command.

   ```sh
   cargo r --bin pywr-v1-validator -- export-schema model.schema.json
   ```

`--kind` selects `model` (the default), `multi-model` or `network`. The same schemas are available to
Rust code from `pywr_v1_schema::json_schema`.

Each release of the `pywr-v1-schema` package attaches the `model` and `multi-model` schemas to its
GitHub release as `pywr-v1-model.schema.json` and `pywr-v1-multi-model.schema.json`. The strict and
network variants are not attached.

A schema describes the shape of a file, not whether the model is valid. References between nodes,
parameters and tables, unique names and the contents of external files are not checked.

Nodes and parameters fall back to a custom type when a core definition fails to deserialise, so
by default a core node or parameter with a misspelt field is still valid. Pass `--strict` to
require a custom node or parameter to use a type that is not a core type, so that such a definition
is reported. It is then reported whether it is invalid or uses a feature this crate does not model,
so a failure under `--strict` is not by itself an error in the model.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [x] Initial support for nodes.
- [x] Initial support for parameters.
- [ ] Initial support for recorders.
- [ ] Schema definitions for all nodes.
- [ ] Schema definitions for all parameters.
- [ ] Schema definitions for all recorders.
- [ ] Python support.


See the [open issues](https://github.com/pywr/pywr-schema/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Apache-2 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/pywr/pywr-schema](https://github.com/pywr/pywr-schema)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/pywr/pywr-schema.svg?style=for-the-badge
[contributors-url]: https://github.com/pywr/pywr-schema/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/pywr/pywr-schema.svg?style=for-the-badge
[forks-url]: https://github.com/pywr/pywr-schema/network/members
[stars-shield]: https://img.shields.io/github/stars/pywr/pywr-schema.svg?style=for-the-badge
[stars-url]: https://github.com/pywr/pywr-schema/stargazers
[issues-shield]: https://img.shields.io/github/issues/pywr/pywr-schema.svg?style=for-the-badge
[issues-url]: https://github.com/pywr/pywr-schema/issues
[license-shield]: https://img.shields.io/github/license/pywr/pywr-schema.svg?style=for-the-badge
[license-url]: https://github.com/pywr/pywr-schema/blob/master/LICENSE.txt

