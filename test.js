const c = require('./bpeRanks/cl100k_base');
const file = 'src/tokenizer/cl100k.jsonl';
const writer = Bun.file(file).writer();
const encoder = new TextEncoder();
// const segmenter = new Intl.Segmenter('en', { granularity: 'grapheme' });

function grapheme (text) {
    const characters = text.split('')
    
    const segments = segmenter.segment('👨‍👩‍👧‍👦'); // Family emoji
    for (const segment of segments) {
        console.log(segment.segment); // Outputs '👨‍👩‍👧‍👦' as one segment
    }
}

for (let idx = 0; idx < c.default.length; idx++) {
    await writer.write(encoder.encode(JSON.stringify({[c.default[idx]]: idx}) + "\n"));
}
await writer.end(); // Close the stream
console.log("✅ Finished writing array to file:", file);
