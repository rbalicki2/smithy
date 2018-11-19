watch :
	RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo watch -x fmt -x '+nightly build'

watch-test :
	RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo watch -x fmt -s 'cd packages/smd_tests && cargo +nightly test'

watch-test-nc :
	RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo watch -x fmt -s 'cd packages/smd_tests && cargo +nightly test -- --nocapture'