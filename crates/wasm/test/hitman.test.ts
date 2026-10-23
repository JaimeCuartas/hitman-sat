import { test } from "node:test";
import assert from "node:assert/strict";
import { Hitman } from "../pkg/hitman_sat_wasm.js";

function enumerateHittingSets(sets: number[][]): number[][] {
  const hitman = new Hitman();
  for (const set of sets) {
    hitman.hit(new Uint32Array(set));
  }
  const found: number[][] = [];
  let hs: Uint32Array | undefined;
  while ((hs = hitman.get()) !== undefined) {
    found.push(Array.from(hs).sort());
    hitman.block(hs);
  }
  hitman.free();
  return found.sort();
}

test("enumerate mhs of [1,2], [2,3], [3]", () => {
  assert.deepEqual(enumerateHittingSets([[1, 2], [2, 3], [3]]), [
    [1, 3],
    [2, 3],
  ]);
});

test("hitting set of empty collection", () => {
  const hitman: Hitman = new Hitman();
  const hs: Uint32Array | undefined = hitman.get();
  const expeted_empty: Uint32Array = new Uint32Array();
  assert.deepEqual(hs, expeted_empty);
});

test("enumerate mhs hitting empty set after", () => {
  const hitman: Hitman = new Hitman();
  hitman.hit(new Uint32Array([1, 2]));
  hitman.hit(new Uint32Array([]));

  assert.deepEqual(hitman.get(), undefined);
  hitman.free();
});

test("enumerate mhs hitting empty set before", () => {
  const hitman: Hitman = new Hitman();
  hitman.hit(new Uint32Array([]));
  hitman.hit(new Uint32Array([1, 2]));

  assert.deepEqual(hitman.get(), undefined);
  hitman.free();
});

test("call hitman after free it", () => {
  const hitman: Hitman = new Hitman();
  hitman.hit(new Uint32Array([1, 2]));
  hitman.free();
  assert.throws(() => hitman.get());
});
