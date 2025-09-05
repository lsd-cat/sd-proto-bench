import init, { encrypt_decrypt_bench, submit_bench } from "./pkg/securedrop_protocol.js";

async function run() {
  await init();
  const iterations = 100;
  console.time('encrypt_decrypt');
  encrypt_decrypt_bench(iterations);
  console.timeEnd('encrypt_decrypt');
  console.time('submit');
  submit_bench(iterations);
  console.timeEnd('submit');
}

run();
