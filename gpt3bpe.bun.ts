import { dlopen, suffix, JSCallback } from "bun:ffi";
const FOREIGN_INTERFACE = import.meta.resolve(`./target/aarch64-apple-darwin/debug/libgpt3bpe.${suffix}`);

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    encode_ffi: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    decode_ffi: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
} as const;

interface Resolver {
    index: bigint
    value: number
}

export function GPT3Encode (buffer: Uint8Array): Uint16Array{
    const pointer: Array<Resolver> = [];
    const callback = new JSCallback(
        (index: bigint, value: number): void => {
        pointer.push({index, value})
        },
        {
            args: ["usize", "u16"],
            returns: "void"
        }
    );
    
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.encode_ffi(
        buffer,
        buffer.length,
        callback
    );
    DYLIB.close();

    return Uint16Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.index < b.index) ? -1 : ((a.index > b.index) ? 1 : 0))
        .map((v, _index, _array) =>  v.value)
    )
};

export function GPT3Decode (buffer: Uint16Array): Uint8Array {
    const pointer: Array<Resolver> = [];
    const callback = new JSCallback(
        (index: bigint, value: number): void => {
        pointer.push({index, value})
        },
        {
            args: ["usize", "u16"],
            returns: "void"
        }
    );

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.decode_ffi(
        buffer,
        buffer.length,
        callback
    )
    DYLIB.close();
    return Uint8Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.index < b.index) ? -1 : ((a.index > b.index) ? 1 : 0))
        .map((v, _index, _array) => v.value)
    )
};

import { equal , } from "bun:assert";
const test = "Hello 👋🏿 world 🌍"

const encoding = GPT3Encode(new TextEncoder().encode(test));
const decoding = new TextDecoder().decode(GPT3Decode(encoding));
equal(test, decoding)

console.log(`Encode: '${test}' -> ${encoding}`);
console.log(`Decode: '${encoding}' -> ${decoding}`);
