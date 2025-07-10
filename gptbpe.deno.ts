function suffix() {
    const os = Deno.build.os;
    if (os.toLowerCase() == "windows") {
        return "dll";
    }
    if (os.toLowerCase() == "darwin") {
        return "dylib";
    }
    return "so";
};

const FOREIGN_INTERFACE = `./target/aarch64-apple-darwin/release/libgptbpe.${suffix() as string}`;

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    encode_r50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    decode_r50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
} as const;

type SimplePointer = Array <{
    idx: bigint
    value: number
}>

// type SimpleArgument = {
//     parameters: 
// }

type vocabulary = 'r50k' | 'p50k';

export function encode (buffer: Uint8Array, _vocabulary: vocabulary): Uint16Array{
    const pointer: SimplePointer = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void"
    }, function (idx: bigint, value: number): void {
        pointer.push({idx, value})
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.encode_r50k(
        buffer,
        buffer.length,
        callback.pointer
    )
    DYLIB.close();

    return Uint16Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
        .map((v, _index, _array) =>  v.value)
    )
};

export function decode (buffer: Uint16Array, _vocabulary: vocabulary): Uint8Array {
    const pointer: SimplePointer = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void"
    }, (idx: bigint, value: number): void => {
        pointer.push({idx, value})
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.decode_r50k(
        buffer,
        buffer.length,
        callback.pointer
    )
    DYLIB.close();
    return Uint8Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
        .map((v, _index, _array) => v.value)
    )
};

import { assertEquals } from "jsr:@std/assert"
const test = "Hello 👋🏿 world 🌍"

const encoding = encode(new TextEncoder().encode(test), 'r50k');
const decoding = new TextDecoder().decode(decode(encoding, 'r50k'));

assertEquals(test, decoding)

console.log(`Encode: '${test}' -> ${encoding}`);
console.log(`Decode: '${encoding}' -> ${decoding}`);
console.log(`indivisible values. -> ${decode(new Uint16Array([521, 452, 12843, 1988, 82]), 'r50k')}`);
console.log(`indivisible values. -> ${decode(new Uint16Array([521, 452, 271, 10506, 68, 3815]), 'r50k')}`);
console.log(`"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D" -> ${decode(new Uint16Array([31373, 50169, 233, 995, 12520, 234, 235]), 'r50k')}`)
console.log(`"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D" -> ${decode(new Uint16Array([31373, 50169, 233, 995, 220, 172, 253, 234, 235]), 'r50k')}`)