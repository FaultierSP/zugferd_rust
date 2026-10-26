set shell := ["bash", "-euo", "pipefail", "-c"]
set dotenv-load

# Folder with the unpacked ZUGFeRD schema package (one sub-folder per profile).
# Set ZUGFERD_XSD_DIR, or override per call: `just xsd_dir=/path/to/Schema test-schema`
xsd_dir := env("ZUGFERD_XSD_DIR", justfile_directory() / "tests/Schema")

export ZUGFERD_XSD_DIR := xsd_dir

default:
    @just --list

# Unit, integration and doc tests. Schema tests are skipped with a notice if no XSDs are found.
test:
    cargo test

# Unit tests only (in src/)
test-unit:
    cargo test --lib

# Integration tests only (in tests/, includes the schema tests)
test-integration:
    cargo test --test '*'

# Only the XSD validation tests; fails if the schemas or xmllint are missing
test-schema:
    ZUGFERD_REQUIRE_XSD=1 cargo test --test xsd_validation
