format :
	cargo fmt && cargo fmt -- crates/smd_tests/src/*

watch :
	cargo watch -s 'make format' -x '+nightly build'

watch-test :
	cargo watch -s 'make format' -s 'cd crates/smd_tests && cargo +nightly test'

watch-test-nc :
	cargo watch -s 'make format' -s 'cd crates/smd_tests && cargo +nightly test --features=cache-logs -- --nocapture'

watch-docs :
	RUSTDOCFLAGS="-Z unstable-options --index-page $$(pwd)/index.md" cargo watch -s 'rm -f target/doc/index.html && cargo doc -p smithy --no-deps --all-features' -w ./crates/ -w ./index.md

build-docs :
	RUSTDOCFLAGS="-Z unstable-options --index-page $$(pwd)/index.md" cargo doc -p smithy --no-deps --all-features

clear-docs :
	rm -rf target/doc

upload-docs :
	aws s3 sync ./target/doc s3://smithy-rs-site/docs/prod/current --cache-control max-age=0,no-cache --acl public-read
	aws cloudfront create-invalidation  --distribution-id E1159YR865AV4M --paths "/*"

watch-offline :
	cargo watch -s 'make format' -x '+nightly build --offline'