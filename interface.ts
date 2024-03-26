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

let buff = new TextEncoder().encode("Let there be light.");
let pointer = dylib.symbols.text_encode(buff, buff.byteLength);
let encoded = new Deno.UnsafePointerView(pointer).getCString();
console.log(`DEBUG | encoded 'Let there be light.' to '${encoded}'`);

buff = new TextEncoder().encode(encoded);
pointer = dylib.symbols.text_decode(buff, buff.byteLength);
let decoded = new Deno.UnsafePointerView(pointer).getCString();
console.log(`DEBUG | decoded '${encoded}' to '${decoded}'`);

dylib.close();
