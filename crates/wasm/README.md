#Compute minimal hitting sets

It is meant for bundlers such as Vite or webpack

#Installation
`npm install hitman-sat`

#Ussage

import { Hitman } from "hitman-sat";

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

enumerateHittingSets([[1, 2], [2, 3], [3]])

#Expected output:
[[1, 3], [2, 3]]
