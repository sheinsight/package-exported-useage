import test from 'ava'
import { performance } from 'perf_hooks';
import { inspectPackageUsage} from '../index.js'
 


test('xx', (t) => {
  const start = performance.now()
  inspectPackageUsage("shineout","/Users/10015448/GitRepository/csp-new");
  const end = performance.now()
  console.log(`inspectPackageUsage cost ${end - start} ms`);
})