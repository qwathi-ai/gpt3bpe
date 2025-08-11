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

const FOREIGN_INTERFACE = `./target/aarch64-apple-darwin/release/libgpt3bpe.${suffix() as string}`;
// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    encode_p50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    decode_p50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    encode_r50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    decode_r50k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    encode_cl100k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    decode_cl100k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
} as const;

type SimplePointer = Array<{
    idx: bigint
    value: number
}>

type Vocabulary = 'r50k' | 'p50k' | 'p50k_edit' | 'cl100k' | 'o200k';

export function encode(buffer: Uint8Array, vocabulary: Vocabulary): Uint16Array {
    const pointer: SimplePointer = [];

    const callback = new Deno.UnsafeCallback({ 
        parameters: ["usize", "u16"],
        result: "void"
    }, function (idx: bigint, value: number): void {
        pointer.push({ idx, value })
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k':
            DYLIB.symbols.encode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'p50k_edit':
            DYLIB.symbols.encode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'r50k':
            DYLIB.symbols.encode_r50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'cl100k':
            DYLIB.symbols.encode_cl100k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;

        default:
            DYLIB.symbols.encode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
    }
    DYLIB.close();

    return Uint16Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

export function decode(buffer: Uint16Array, vocabulary: Vocabulary): Uint8Array {
    const pointer: SimplePointer = [];

    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void"
    }, (idx: bigint, value: number): void => {
        pointer.push({ idx, value })
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    switch (vocabulary) {
        case 'p50k':
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'p50k_edit':
            DYLIB.symbols.encode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'r50k':
            DYLIB.symbols.decode_r50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
        case 'cl100k':
            DYLIB.symbols.decode_cl100k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;

        default:
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.pointer
            )
            break;
    }
    DYLIB.close();
    return Uint8Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

async function* readLines(path: string) {
    const file = await Deno.readTextFile(path);
    let lines = file.split(/\r?\n/);
    for (const line of lines) {
        yield line
    }
}

import { deepEqual, equal } from "node:assert";
const path = "./src/bpe/vocabulary/tests.jsonl";

for await (let line of readLines(path)) {
    const json = JSON.parse(line) as { encoded: number[], encoding: Vocabulary, sample: string }

    if (json.encoded.length > 0 && json.encoding != 'cl100k' && json.encoding != 'o200k') {
        try {
            const encoding = encode(new TextEncoder().encode(json.sample), json.encoding);
            deepEqual(encoding, Uint16Array.from(json.encoded))
            const decoding = new TextDecoder().decode(decode(Uint16Array.from(json.encoded), json.encoding));
            equal(json.sample, decoding)
            console.log({json})
        } catch (error) {
            console.error({
                error,
                json
            })
        }
    };
}
