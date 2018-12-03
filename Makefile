watch :
	cargo watch -x fmt -x '+nightly build'

watch-test :
	cargo watch -x fmt -s 'cd packages/smd_tests && cargo +nightly test'

watch-test-nc :
	 cargo watch -x fmt -s 'cd packages/smd_tests && cargo +nightly test -- --nocapture'
