import test from 'ava'
import { performance } from 'perf_hooks';
import { inspectPackageUsage} from '../index.js'
 


test('xx', (t) => {
  const start = performance.now()
  const res = inspectPackageUsage("shineout","/Users/10015448/GitRepository/fsp-front");
  console.log(res);
  const end = performance.now()
  console.log(`inspectPackageUsage cost ${end - start} ms`);
  console.log("xx");

})