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
    grapheme: {
        parameters: ["buffer", "u8", "function"],
        result: "void",
    },
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
    encode_o200k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
    decode_o200k: {
        parameters: ["buffer", "u32", "function"],
        result: "void",
    },
} as const;

export function grapheme(buffer: Uint8Array): Uint8Array {
    const safeBuffer = Uint8Array.from(buffer);
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new Deno.UnsafeCallback({ 
        parameters: ["usize", "u8"],
        result: "void"
    }, function (idx: bigint, value: number): void {
        simplePointer.push({ idx, value })
    });
    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.grapheme(
        safeBuffer,
        safeBuffer.length,
        callback.pointer
    )
    DYLIB.close();

    return Uint8Array.from(
        simplePointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

type Vocabulary = 'r50k' | 'p50k' | 'cl100k' | 'o200k';
export function encode(buffer: Uint8Array, vocabulary: Vocabulary = 'p50k'): Uint16Array | Uint32Array {
    const safeBuffer = Uint8Array.from(buffer);
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new Deno.UnsafeCallback({ 
        parameters: ["usize", "u32"],
        result: "void"
    }, function (idx: bigint, value: number): void {
        simplePointer.push({ idx, value })
    });
    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k':
            DYLIB.symbols.encode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        case 'r50k':
            DYLIB.symbols.encode_r50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        case 'cl100k':
            DYLIB.symbols.encode_cl100k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        case 'o200k':
            DYLIB.symbols.encode_o200k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;

        default:
            DYLIB.symbols.encode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
    }
    DYLIB.close();


    return vocabulary in ['r50k', 'p50k'] ? Uint16Array.from(
        simplePointer
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    ) : Uint32Array.from(
        simplePointer
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

export function decode(buffer: Uint16Array | Uint32Array, vocabulary: Vocabulary = 'p50k'): Uint8Array {
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u32"],
        result: "void"
    }, (idx: bigint, value: number): void => {
        simplePointer.push({ idx, value })
    });
    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    
    switch (vocabulary) {
        case 'p50k':{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        }
        case 'r50k':{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_r50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        }
        case 'cl100k':{
            const safeBuffer = Uint32Array.from(buffer)
            DYLIB.symbols.decode_cl100k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        }
        case 'o200k': {
            const safeBuffer = Uint32Array.from(buffer)
            DYLIB.symbols.decode_o200k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        }

        default:{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.pointer
            )
            break;
        }
    }
    DYLIB.close();
    return Uint8Array.from(
        simplePointer
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

async function* readLines(path: string) {
    const file = await Deno.readTextFile(path);
    const lines = file.split(/\r?\n/);
    for (const line of lines) {
        yield line
    }
};

import { deepEqual, equal } from "node:assert";
const path = "./src/bpe/vocabulary/tests.jsonl";
console.log("[INFO]: Running tests...");
let count = 0;
for await (const line of readLines(path)) {
    let json: { encoded: number[], model: Vocabulary, text: string }
    try {
        json = JSON.parse(line)
    } catch(_e) {
        continue
    }
    if (json.encoded.length > 0) {
        const encoding = encode(new TextEncoder().encode(json.text), json.model)
        deepEqual(encoding, json.model in ['r50k', 'p50k'] ? Uint16Array.from(json.encoded) : Uint32Array.from(json.encoded))
        const textDecoding = new TextDecoder().decode(decode(encoding, json.model))
        equal(json.text, textDecoding)
        count += 1
    };
}
console.log("[INFO]: All ",count," tests passed!")