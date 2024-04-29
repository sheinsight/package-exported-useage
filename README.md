# @shined/package-exported-useage 

[![CI](https://github.com/sheinsight/package-exported-useage/actions/workflows/CI.yml/badge.svg)](https://github.com/sheinsight/package-exported-useage/actions/workflows/CI.yml)

Used to count the usage of exported modules in a specific package.

Power by rust .


## How to use

```bash
npm i @shined/package-exported-useage
```

```javascript
const { inspectPackageUsage } = require('@shined/package-exported-usage');
const res = inspectPackageUsage('antd','~/demo-front');
```