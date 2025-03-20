function suffix() {
    const os = Deno.build.os;
    if (os.toLowerCase() == "windows") {
        return "dll";
    }
    if (os.toLowerCase() == "darwin") {
        return "dylib";
    }
    return "so";
}

const FOREIGN_INTERFACE =
    `./target/aarch64-apple-darwin/release/libgpt3bpe.${suffix() as string}`;

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    encode_ffi: {
        parameters: ["pointer", "u32", "function"],
        result: "void",
    },
    decode_ffi: {
        parameters: ["pointer", "u32", "function"],
        result: "void",
    },
} as const;

interface Resolver {
    index: bigint;
    value: number;
}

export function GPT3Encode(buffer: Uint8Array): Uint16Array {
    const pointer: Array<Resolver> = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void",
    }, (index: bigint, value: number): void => {
        pointer.push({ index, value });
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.encode_ffi(
        Deno.UnsafePointer.of(buffer.buffer) as Deno.PointerValue,
        buffer.length,
        callback.pointer,
    );
    DYLIB.close();

    if (pointer.length == 0) {
        throw RangeError(`Buffer value of ${buffer} does not exist as a byte-pair.`, {cause: "Byte-Pairing not found."})
    };

    return Uint16Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) =>
                (a.index < b.index) ? -1 : ((a.index > b.index) ? 1 : 0)
            )
            .map((v, _index, _array) => v.value),
    );
}

export function GPT3Decode(buffer: Uint16Array): Uint8Array {
    const pointer: Array<Resolver> = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void",
    }, (index: bigint, value: number): void => {
        pointer.push({ index, value });
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.decode_ffi(
        Deno.UnsafePointer.of(buffer.buffer) as Deno.PointerValue,
        buffer.length,
        callback.pointer,
    );
    DYLIB.close();
    return Uint8Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) =>
                (a.index < b.index) ? -1 : ((a.index > b.index) ? 1 : 0)
            )
            .map((v, _index, _array) => v.value),
    );
}

// NP -> Det N | N | Pn | Det A N | A NP

import { assertEquals } from "jsr:@std/assert";
// const test = "Hello 👋🏿 world 🌍";
const test = "very";

let bytes = new TextEncoder().encode(test)
// console.log(`Text encoding: '${test}' -> ${bytes}`);
const tokens = GPT3Encode(bytes);
console.log(`GPT encoding: '${test}' -> ${tokens}`);
bytes = GPT3Decode(tokens)
// console.log(`GPT decoding: '${tokens}' -> ${bytes}`);
const text = new TextDecoder().decode(bytes);
console.log(`Decode: '${tokens}' -> ${text}\n`);
assertEquals(test, text)