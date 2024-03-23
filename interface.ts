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
    parameters: ["buffer"],
    result: "pointer",
    // nonblocking: true,
  },
} as const;

console.log(`target/debug/libamile.${suffix() as string}`);
console.log(symbols);

const dylib = Deno.dlopen(
  `target/debug/libamile.${suffix() as string}`,
  symbols
);

const buff = new TextEncoder().encode("How are you?");
const pointer = dylib.symbols.text_encode(buff);
console.log(
  `Encode 'How are you?' : ${new Deno.UnsafePointerView(pointer).getCString()}`
);
