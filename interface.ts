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
  text_encode: {
    parameters: ["buffer", "usize"],
    result: "buffer",
  },
  text_decode: {
    parameters: ["buffer", "usize"],
    result: "buffer",
  },
} as const;
console.log("[DEBUG]: ", FOREIGN_INTERFACE, "\n", SYMBOLS);

const textEncode = (text: string): Uint32Array => {
  const dylib = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
  const buffer = new TextEncoder().encode(text);
  const pointer = dylib.symbols.text_encode(buffer, buffer.byteLength);
  const _buffer = new Uint32Array();
  new Deno.UnsafePointerView(pointer as Deno.PointerObject).copyInto(_buffer);
  dylib.close();
  return _buffer;
};

const textDecode = (buffer: Uint32Array): string => {
  const dylib = Deno.dlopen(FOREIGN_INTERFACE, SYMBOLS);
  const pointer = dylib.symbols.text_decode(buffer, buffer.byteLength);
  const _buffer = new Uint8Array();
  new Deno.UnsafePointerView(pointer as Deno.PointerObject).copyInto(_buffer);
  dylib.close();
  return new TextDecoder().decode(_buffer);
};

console.log("Deno Encoding: ", textEncode("let there be light.")),
  console.log(
    "Deno Decoding: ",
    textDecode(new Uint32Array([1616, 612, 307, 1657, 13]))
  ),
  Deno.test("Decode 'let there be light'", () => {
    textEncode("let there be light.") ==
      new Uint32Array([1616, 612, 307, 1657, 13]);
    textDecode(new Uint32Array([1616, 612, 307, 1657, 13])) ==
      "let there be light.";
  });
