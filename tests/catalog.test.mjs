import assert from 'node:assert/strict';
import test from 'node:test';
import { readFileSync } from 'node:fs';
import { describeApi } from '../apps/api/src/index.mjs';
import { routes, product } from '../packages/contracts/src/index.mjs';
const catalog = JSON.parse(readFileSync('catalog.json', 'utf8'));
test('catalog links split repositories', () => { assert.equal(catalog.org, "embedded-alerts"); assert.equal(catalog.repos.marketing, "embedded-alerts/embedded-alerts.github.io"); assert.ok(catalog.apps.includes('api')); assert.ok(catalog.packages.includes('contracts')); });
test('api descriptor and contracts agree', () => { assert.equal(product, "embedded-alerts"); const api = describeApi(); assert.ok(api.routes.every((route) => routes.includes(route))); });
