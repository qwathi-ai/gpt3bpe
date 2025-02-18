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
const SYMBOLS = {
    encode_ffi: {
        parameters: ["buffer", "u8", "function"],
        result: "void",
    },
    decode_ffi: {
        parameters: ["buffer", "u8", "function"],
        result: "void",
    },
} as const;

interface Resolver {
    index: bigint
    value: number
}

export function GPT3Encode (buffer: Uint8Array): Uint16Array{
    const pointer: Array<Resolver> = [];
    const callback = new Deno.UnsafeCallback({
        parameters: ["usize", "u16"],
        result: "void"
    }, (index: bigint, value: number): void => {
        pointer.push({index, value})
    });
    
    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.encode_ffi(
        buffer,
        buffer.length,
        callback.pointer
    )
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
    const callback = new Deno.UnsafeCallback({
        parameters: ["u64", "u16"],
        result: "void"
    }, (index: bigint, value: number): void => {
        pointer.push({index, value})
    });

    const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
    DYLIB.symbols.decode_ffi(
        buffer,
        buffer.length,
        callback.pointer
    )
    DYLIB.close();
    return Uint8Array.from(
        pointer
        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt#comparisons for sorting bigint
        .sort((a, b) => (a.index < b.index) ? -1 : ((a.index > b.index) ? 1 : 0))
        .map((v, _index, _array) => v.value)
    )
};

const test = "let there be light."

const encoding = GPT3Encode(new TextEncoder().encode(test));
console.log(`Encode: '${test}' -> ${encoding}`);
console.log(`Decode: '${encoding}' -> ${new TextDecoder().decode(GPT3Decode(encoding))}`);