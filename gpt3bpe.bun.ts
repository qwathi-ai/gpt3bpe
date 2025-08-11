import { dlopen, suffix, JSCallback } from "bun:ffi";
const FOREIGN_INTERFACE = import.meta.resolve(`./target/aarch64-apple-darwin/debug/libgpt3bpe.${suffix}`);

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
} as const;

type SimplePointer = Array<{
    idx: bigint
    value: number
}>

export function grapheme(buffer: Uint8Array): Uint8Array {
    const pointer: SimplePointer = [];
    const callback = new JSCallback(function (idx: bigint, value: number): void {
        pointer.push({ idx, value })
    }, {
        args: ["usize", "u8"],
        returns: "void"
    });

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.grapheme(
        buffer,
        buffer.length,
        callback.ptr
    );
    DYLIB.close();

    return Uint8Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

type Vocabulary = 'r50k' | 'p50k' | 'p50k_edit' | 'cl100k' | 'o200k';

export function encode(buffer: Uint8Array, vocabulary: Vocabulary): Uint16Array {
    const pointer: SimplePointer = Array(buffer.length);
    const callback = new JSCallback(function (idx: bigint, value: number): void {
        pointer.push({ idx, value })
    }, {
        args: ["usize", "u32"],
        returns: 'void'
    });

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k':
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;

        case 'p50k_edit':
            DYLIB.symbols.encode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            )
            break;

        case 'r50k':
            DYLIB.symbols.decode_r50k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;

        case 'cl100k':
            DYLIB.symbols.decode_cl100k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;
        default:
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            );  
            break;
    }
    DYLIB.close();

    return Uint16Array.from(
        pointer
            // // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            // .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

export function decode(buffer: Uint16Array, vocabulary: Vocabulary): Uint8Array {
    const pointer: SimplePointer = [];

    const callback = new JSCallback(function (idx: bigint, value: number): void {
        pointer.push({ idx, value })
    }, {
        args: ["usize", "u8"],
        returns: "void"
    });

    const DYLIB = dlopen(FOREIGN_INTERFACE, SYMBOLS);

    switch (vocabulary) {
        case 'p50k':
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;
            
        case 'p50k_edit':
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;

        case 'r50k':
            DYLIB.symbols.decode_r50k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;

        case 'cl100k':
            DYLIB.symbols.decode_cl100k(
                buffer,
                buffer.length,
                callback.ptr
            );   
            break;
        default:
            DYLIB.symbols.decode_p50k(
                buffer,
                buffer.length,
                callback.ptr
            );  
            break;
    }
    DYLIB.symbols.decode_p50k(
        buffer,
        buffer.length,
        callback.ptr
    )
    DYLIB.close();
    return Uint8Array.from(
        pointer
            // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
            .sort((a, b) => (a.idx < b.idx) ? -1 : ((a.idx > b.idx) ? 1 : 0))
            .map((v, _index, _array) => v.value)
    )
};

async function *readLines(path: string) {
    const reader = Bun.file(path).stream().pipeThrough(new TextDecoderStream('utf-8')).getReader();
    let remainder = ''
    while(true) {
        const {value, done} = await reader.read()
        if(done) break
        let lines = (remainder + value).split(/\r?\n/)
        remainder = lines.pop()!
        for(const line of lines) {
            yield line
        }
    }

    if(remainder) {
        yield remainder
    }
}

import { equal, deepEqual } from "bun:assert";
const path = "./src/bpe/vocabulary/tests.jsonl";

for await (let line of readLines(path)) {
    const json = JSON.parse(line) as {"encoded": number[], encoding: Vocabulary, sample: string}
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