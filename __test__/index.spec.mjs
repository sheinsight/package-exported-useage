import test from 'ava'
import { inspectPackageUsage} from '../index.js'
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);


test('should to be 0 when only import', async (t) => {
  
  const workspace = path.join(__dirname,"fixtures/demo1");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 0);
})

test('should to be 1 when import and use by identity', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo2");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})

test('should to be 1 when normal react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo3");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})


test('should to be 1 when selfClosing react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo4");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})


test('should to be 1 when default import normal react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo5");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})

test('should to be 1 when default import selfClosing react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo6");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})

test('should to be 1 when namespace import normal react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo7");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})

test('should to be 1 when namespace import selfClosing react component', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo8");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})

test('should to be 2 when 2 libs', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo9");
  const res = await inspectPackageUsage(workspace,["antd","lodash"]);
  t.is(res.length, 2);
})

test('should to be 1 when 2 libs but only scan 1 lib', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo9");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})
 
test('should to be 1 when 2 libs but no lodash imported', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo11");
  const res = await inspectPackageUsage(workspace,["antd","lodash"]);
  t.is(res.length, 1);
})
 

test('should to be 1 when use React.createElement', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo12");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 1);
})



test('should to be 2 when use alias', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo13");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 2);
})
 
test('should to be 4 when directory name end with .ts', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo14");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 4);
})

test('should to be 4 when valid UTF-8', async (t) => {
  const workspace = path.join(__dirname,"fixtures/demo15");
  const res = await inspectPackageUsage(workspace,["antd"]);
  t.is(res.length, 0);
})