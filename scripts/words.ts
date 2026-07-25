import { readLines } from '../sdk/bun.encoder.ts';
import { load } from '../sdk/bun.embeddings.ts';

const SOURCE = '/Users/athenkosimase/.amile/words.txt';
const QUEUE_SIZE = 10000;
let QUEUE: {buffer: Uint8Array, embedding: Float32Array }[] = [];
let COUNT = 0;
let inserted = 0;


async function clearing () {
    for await (const b of load(QUEUE)) {
        if (b) {
            inserted += 1;
        }
    }
    COUNT += 1
    console.log(`[INFO]: ${COUNT} batches processed, ${inserted} rows inserted.`);
    QUEUE = [];
};


for await (const line of readLines(SOURCE)) {
    try {
        const [text, ...vector] = line.split(" ")
        const embedding = Float32Array.from(vector);
        if (embedding.length != 300) {
            throw new Error("Embedding space not aligned. Expect a 300 dimensional vector.")
        }
        const buffer = new TextEncoder().encode(text);
        QUEUE.push({buffer, embedding});
    } catch (_) {
        console.warn("[WARNING]: Could not process \n", line, "\n", _)
        continue;
    }

    if (QUEUE.length >= QUEUE_SIZE) {
        await clearing()
        continue;
    }
}

await clearing()