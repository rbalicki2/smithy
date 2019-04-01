format :
	cargo fmt && cargo fmt -- crates/smd_tests/src/*

watch :
	cargo watch -s 'make format' -x '+nightly build'

watch-test :
	cargo watch -s 'make format' -s 'cd crates/smd_tests && cargo +nightly test'

watch-test-nc :
	 cargo watch -s 'make format' -s 'cd crates/smd_tests && cargo +nightly test -- --nocapture'
