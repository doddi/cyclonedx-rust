# cyclonedx-rust

[![Build Status](https://github.com/doddi/cyclonedx-rust/actions/workflows/CI.yml/badge.svg)](https://github.com/doddi/cyclonedx-rust/actions/workflows/CI.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-brightgreen.svg)](LICENSE)

cyclonedx-rust is a simple library to encode/decode CycloneDX BOM


You can encode and decode from any reader type using the methods:

Decoding:
`CycloneDX::decode(reader: R, format: CycloneDXFormatType,) -> Result<CycloneDX, CycloneDXDecodeError>`


Encoding:
`CycloneDX::encode<(writer: W, dx: CycloneDX, format: CycloneDXFormatType,) -> Result<(), CycloneDXEncodeError>`
