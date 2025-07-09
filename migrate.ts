import tokens from './bpeRanks/p50k_base';
import { grapheme } from './gptbpe.bun'

const filePath = 'src/tokenizer/vocabulary/p50k.jsonl';
const writer = Bun.file(filePath).writer();
const e = new TextEncoder();
const d = new TextDecoder();

for (let idx = 0; idx < tokens.length; idx++) {
    const whitespace_removed = d.decode(
        grapheme(
            e.encode(
                JSON.stringify(tokens[idx], function(key, value) {
                    return value.replace(/[^\w\s]/gi, '');
                })
            )
        )
    );
    const jsonl = JSON.stringify({[whitespace_removed]: idx});
    await writer.write(e.encode(jsonl + "\n"));
};

await writer.end(); // Close the stream
console.log("✅ Finished writing array to file:", filePath);
