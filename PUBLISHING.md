# Notes before publishing

Due to using submodules and these submodules containing Cargo.toml, they will always be ignored by cargo, so we have to delete each Cargo.toml from languages/.

Use the script remove-tomls.sh
