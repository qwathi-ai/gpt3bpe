import { dlopen, suffix, JSCallback } from "bun:ffi";

const FOREIGN_INTERFACE = import.meta.resolve(`./target/aarch64-apple-darwin/debug/libgptbpe.${suffix}`);

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    grapheme: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    encode_r50k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    decode_r50k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
} as const;

type SimplePointer = Array <{
    idx: bigint
    value: number
}>

type vocabulary = 'r50k' | 'p50k';

export function grapheme (buffer: Uint8Array): Uint8Array{
    const pointer: SimplePointer = [];
    const callback = new JSCallback( function (idx: bigint, value: number): void {
        pointer.push({idx, value})
    },{
        args: ["usize", "u16"],
        returns: "void"
    });
    
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.grapheme(
        buffer,
        buffer.length,
        callback
    );
    DYLIB.close();

    return Uint8Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
        .map((v, _index, _array) =>  v.value)
    )
};


export function encode (buffer: Uint8Array, _vocabulary: vocabulary): Uint16Array{
    const pointer: SimplePointer = Array(buffer.length);
    const callback = new JSCallback( function (idx: bigint, value: number): void {
        pointer.push({idx, value})
    },{
        args: ["usize", "u16"],
        returns: "void"
    });
    
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.encode_r50k(
        buffer,
        buffer.length,
        callback
    );
    DYLIB.close();

    return Uint16Array.from(
        pointer
        // // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        // .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
        .map((v, _index, _array) =>  v.value)
    )
};

export function decode (buffer: Uint16Array, _vocabulary: vocabulary): Uint8Array {
    const pointer: SimplePointer = [];
    const callback = new JSCallback( function (idx: bigint, value: number): void {
        pointer.push({idx, value})
    },{
        args: ["usize", "u16"],
        returns: "void"
    });

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.decode_r50k(
        buffer,
        buffer.length,
        callback
    )
    DYLIB.close();
    return Uint8Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
        .map((v, _index, _array) => v.value)
    )
};

import { equal , } from "bun:assert";
const test = "Hello 👋🏿 world 🌍"

const encoding = encode(new TextEncoder().encode(test), 'r50k');
const decoding = new TextDecoder().decode(decode(encoding, 'r50k'));
equal(test, decoding)

console.log(`Encode: '${test}' -> ${encoding}`);
console.log(`Decode: '${encoding}' -> ${decoding}`);
