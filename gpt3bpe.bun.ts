import { dlopen, suffix, JSCallback } from "bun:ffi";
const FOREIGN_INTERFACE = import.meta.resolve(`./target/aarch64-apple-darwin/release/libgpt3bpe.${suffix}`);

// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Errors/Invalid_array_length for max ArrayBuffer length.
const SYMBOLS = {
    grapheme: {
        args: ["buffer", "u8", "function"],
        returns: "void",
    },
    encode_p50k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    decode_p50k: {
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
    encode_cl100k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    decode_cl100k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    encode_o200k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    decode_o200k: {
        args: ["buffer", "u32", "function"],
        returns: "void",
    },
    embed_p50k: {
        args: ["buffer", "u32", "buffer", "u32"],
        returns: "bool",
    },
    embed_r50k: {
        args: ["buffer", "u32", "buffer", "u32"],
        returns: "bool",
    },
    embed_cl100k: {
        args: ["buffer", "u32", "buffer", "u32"],
        returns: "bool",
    },
    embed_o200k: {
        args: ["buffer", "u32", "buffer", "u32"],
        returns: "bool",
    },
} as const;

export function grapheme(buffer: Uint8Array): Uint8Array {
    const safeBuffer = Uint8Array.from(buffer);
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new JSCallback(function (idx: bigint, value: number): void {
        simplePointer.push({ idx, value })
    }, {
        args: ["usize", "u8"],
        returns: "void"
    });

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.grapheme(
        safeBuffer,
        safeBuffer.length,
        callback.ptr
    );
    DYLIB.close();

    return Uint8Array.from(
        simplePointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

export type Vocabulary = 'r50k' | 'p50k' | 'cl100k' | 'o200k';

export function encode(buffer: Uint8Array, vocabulary: Vocabulary): Uint16Array | Uint32Array {
    const safeBuffer = Uint8Array.from(buffer);
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new JSCallback(function (idx: bigint, value: number): void {
        simplePointer.push({ idx, value })
    }, {
        args: ["usize", "u32"] ,
        returns: "void"
    });
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k': 
            DYLIB.symbols.encode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            );   
            break;
        case 'r50k':
            DYLIB.symbols.encode_r50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            );   
            break;

        case 'cl100k':
            DYLIB.symbols.encode_cl100k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            );   
            break;
        case 'o200k':
            DYLIB.symbols.encode_o200k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            );
            break;
        default:
            DYLIB.symbols.encode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            );  
            break;
    }
    DYLIB.close();

    return vocabulary in [] ? Uint16Array.from(
        simplePointer
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    ) : Uint32Array.from(
        simplePointer
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    ) 
};

export function decode(buffer: Uint16Array | Uint32Array, vocabulary: Vocabulary): Uint8Array {
    const simplePointer: {idx: bigint, value: number}[] = [];
    const callback = new JSCallback(function (idx: bigint, value: number): void {
        simplePointer.push({ idx, value })
    }, {
        args: ["usize", "u8"] ,
        returns: "void"
    });
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k':{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            )
            break;
        }
        case 'r50k':{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_r50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            )
            break;
        }
        case 'cl100k':{
            const safeBuffer = Uint32Array.from(buffer)
            DYLIB.symbols.decode_cl100k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            )
            break;
        }
        case 'o200k': {
            const safeBuffer = Uint32Array.from(buffer)
            DYLIB.symbols.decode_o200k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
            )
            break;
        }

        default:{
            const safeBuffer = Uint16Array.from(buffer)
            DYLIB.symbols.decode_p50k(
                safeBuffer,
                safeBuffer.length,
                callback.ptr
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

export function embed(buffer: Uint8Array, embedding: Float32Array, vocabulary: Vocabulary): boolean{
    let simplePointer: boolean;
    const safeBuffer = Uint8Array.from(buffer);
    const safeEmbedding = Float32Array.from(embedding);
    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    switch (vocabulary) {
        case 'p50k':{
            simplePointer = DYLIB.symbols.embed_p50k(
                safeBuffer,
                safeBuffer.length,
                safeEmbedding,
                safeEmbedding.length
            )
            break;
        }
        case 'r50k':{
            simplePointer = DYLIB.symbols.embed_r50k(
                safeBuffer,
                safeBuffer.length,
                safeEmbedding,
                safeEmbedding.length
            )
            break;
        }
        case 'cl100k':{
            simplePointer = DYLIB.symbols.embed_cl100k(
                safeBuffer,
                safeBuffer.length,
                safeEmbedding,
                safeEmbedding.length
            )
            break;
        }
        case 'o200k':{
            simplePointer = DYLIB.symbols.embed_o200k(
                safeBuffer,
                safeBuffer.length,
                safeEmbedding,
                safeEmbedding.length
            )
            break;
        }
        default:{
            simplePointer = DYLIB.symbols.embed_p50k(
                safeBuffer,
                safeBuffer.length,
                safeEmbedding,
                safeEmbedding.length
            )
            break;
        }
    }
    DYLIB.close();
    return simplePointer
}

import { equal, deepEqual } from "node:assert";
export async function *readLines(path: string) {
    const reader = Bun.file(path).stream().pipeThrough(new TextDecoderStream('utf-8'), {}).getReader();
    let remainder = ''
    while(true) {
        const {value, done} = await reader.read()
        if(done) break
        const lines = (remainder + value).split(/\r?\n/)
        remainder = lines.pop()!
        for(const line of lines) {
            yield line
        }
    }

    if(remainder) {
        yield remainder
    }
}

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
