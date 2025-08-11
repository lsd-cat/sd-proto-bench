const { chromium, firefox } = require('playwright');
const http = require('http');
const fs = require('fs');
const path = require('path');

function startServer(root) {
  return new Promise(resolve => {
    const server = http.createServer((req, res) => {
      const filePath = path.join(root, req.url);
      fs.readFile(filePath, (err, data) => {
        if (err) {
          res.statusCode = 404;
          res.end('Not found');
          return;
        }
        const ext = path.extname(filePath);
        const types = { '.html': 'text/html', '.js': 'application/javascript', '.wasm': 'application/wasm' };
        res.setHeader('Content-Type', types[ext] || 'application/octet-stream');
        res.end(data);
      });
    }).listen(0, () => resolve(server));
  });
}

async function runBenchmark(browserType) {
  const server = await startServer(__dirname);
  const port = server.address().port;
  const url = `http://localhost:${port}/www/index.html`;
  const browser = await browserType.launch();
  const page = await browser.newPage();
  await page.goto(url);
  await page.waitForFunction(() => window.generate_x25519_keypair);
  // Run a small benchmark inside the page context
  const duration = await page.evaluate(() => {
    const msg = new TextEncoder().encode('hello');
    const iterations = 100;
    const start = performance.now();
    for (let i = 0; i < iterations; i++) {
      const kp = window.generate_x25519_keypair();
      window.hpke_encrypt(kp.public_key, msg);
    }
    return performance.now() - start;
  });
  await browser.close();
  server.close();
  return { browser: browserType.name(), duration };
}

(async () => {
  for (const type of [chromium, firefox]) {
    const result = await runBenchmark(type);
    console.log(`${result.browser}: ${result.duration.toFixed(2)}ms for 100 iterations`);
  }
})();
