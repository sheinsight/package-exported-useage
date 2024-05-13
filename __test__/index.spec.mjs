import test from 'ava'
import { performance } from 'perf_hooks';
import { inspectPackageUsage} from '../index.js'
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);


test('should to be array of len is 4', (t) => {
  const start = performance.now()
  // let errorCount = 0;
  const res = inspectPackageUsage("shineout",path.join(__dirname,"fixtures"),(v) => {
    console.log("--->",v);
    // errorCount++;
  });
  const end = performance.now()
  console.log(`inspectPackageUsage cost ${end - start} ms`);
  t.is(res.length, 4);
  // t.is(errorCount, 1);
})