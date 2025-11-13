# ethproofs-verifier


First - download the airbender proof from ethproofs.

Run:

```
RUST_MIN_STACK=267108864 cargo run --release -- --input-file ~/Downloads/matter-labs_f9c5e994-96fc-45c0-98fe-835a53313a62_705856.bin
```

You'll get the output like this:

```
End params (recursion verification key): [592082644, 914832686, 2098567393, 2326800689, 2139187838, 4177631951, 1218871476, 1805766682]
Basic program (verification key): [2515378349, 4234595854, 3925512183, 4162864624, 1989541961, 1232249954, 1448623067, 1605092683]
Public input (per block): [2820680802, 124999842, 2275559499, 200691927, 3104886037, 3143080570, 941459835, 587801584]
```

The first 2 things are verification keys (for recursion and for the 'basic' program).
In theory they should be always 'constant' (and you should build the program + recursion program and check that they match).

The last entry (public input), would differ for each block, and it represents the hash of the commitment to the block (that contains information like new & old state roots etc).
