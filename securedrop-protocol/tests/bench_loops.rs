use securedrop_protocol::bench::{encrypt_decrypt_bench, submit_bench};

#[test]
fn run_encrypt_decrypt_bench_loop() {
    encrypt_decrypt_bench(1);
}

#[test]
fn run_submit_bench_loop() {
    submit_bench(1);
}
