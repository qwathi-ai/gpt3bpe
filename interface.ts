const suffix = () => {
  const os = Deno.build.os;
  if (os.toLowerCase() == "windows") {
    return "dll";
  }
  if (os.toLowerCase() == "darwin") {
    return "dylib";
  }
  return "so";
};

const FOREIGN_INTERFACE = `target/debug/libamile.${suffix() as string}`;
const SYMBOLS = {
  text_encode_from_buffer: {
    parameters: ["buffer", "usize"],
    result: "buffer",
  },
  text_decode_from_buffer: {
    parameters: ["buffer", "usize"],
    result: "buffer",
  },
} as const;
console.log("[DEBUG]: ", FOREIGN_INTERFACE, "\n", SYMBOLS);
const DYLIB = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);

const textEncode = (buffer: Uint8Array): Int32Array => {
  const pointer = DYLIB.symbols.text_encode_from_buffer(
    buffer,
    buffer.byteLength,
  );
  const _buffer = new Int32Array();
  new Deno.UnsafePointerView(
    pointer as Deno.PointerValue<Int32Array> as Deno.PointerObject<Int32Array>,
  ).copyInto(_buffer);
  console.log("[DEBUG]: ", pointer, _buffer);
  return _buffer;
};

const textDecode = (buffer: Int32Array): Uint8Array => {
  const pointer = DYLIB.symbols.text_decode_from_buffer(buffer, buffer.length);
  const _buffer = new Uint8Array();
  new Deno.UnsafePointerView(
    pointer as Deno.PointerValue<Uint8Array> as Deno.PointerObject<Uint8Array>,
  ).copyInto(_buffer);
  console.log("[DEBUG]: ", pointer, _buffer);
  return _buffer;
};

// import { assert } from "https://deno.land/std@0.224.0/assert/mod.ts";

// Deno.test("Test Encoding", () => {
//   assert(textEncode("let there be light.") === new Int32Array([1616, 612, 307, 1657, 13]));
// });

textEncode(new TextEncoder().encode("let there be light."));
textDecode(new Int32Array([1616, 612, 307, 1657, 13]));
DYLIB.close();
