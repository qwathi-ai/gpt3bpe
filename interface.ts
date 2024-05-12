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

const symbols = {
  text_encode: {
    parameters: ["buffer", "usize"],
    result: "pointer",
    // nonblocking: true,
  },
  text_decode: {
    parameters: ["buffer", "usize"],
    result: "pointer",
    // nonblocking: true,
  },
} as const;

console.log(`target/debug/libamile.${suffix() as string}`);
const dylib = Deno.dlopen(
  `target/debug/libamile.${suffix() as string}`,
  symbols
);

const buff = new TextEncoder().encode("let there be light.");
// let buff = new TextEncoder().encode("Hello");
let pointer = dylib.symbols.text_encode(buff, buff.byteLength);
const encoded: string = new Deno.UnsafePointerView(pointer).getPointer();
console.log(`DEBUG | encoded 'let there be light.' to '${encoded}'`);
// [1616, 612, 307, 1657, 13]

buff = new TextEncoder().encode(encoded);
pointer = dylib.symbols.text_decode(buff, buff.byteLength);
const decoded = new Deno.UnsafePointerView(pointer).getPointer();
console.log(`DEBUG | decoded '${encoded}' to '${decoded}'`);

dylib.close();
