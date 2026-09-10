// merge-text-files.mjs
// Usage:
//   node merge-text-files.mjs output.txt input1.txt input2.txt ...

import fs from "node:fs";
import path from "node:path";

function usageAndExit() {
    console.error(
        "Usage: node merge-text-files.mjs output.txt input1.txt input2.txt ...",
    );
    process.exit(1);
}

const args = process.argv.slice(2);
if (args.length < 2) usageAndExit();

const outFile = args[0];
const inputFiles = args.slice(1);

let merged = "";
merged += `MERGED FILE\n`;
merged += `Generated: ${new Date().toISOString()}\n`;
merged += `Inputs: ${inputFiles.length}\n\n`;

for (const file of inputFiles) {
    const absolute = path.resolve(process.cwd(), file);

    let content;
    try {
        content = fs.readFileSync(absolute, "utf8");
    } catch (err) {
        console.error(`Failed to read: ${file}`);
        throw err;
    }

    const fileNameOnly = path.basename(file);

    merged += `===== FILE: ${fileNameOnly} =====\n`;
    merged += content;

    // Ensure files don’t run together if the source doesn’t end with newline.
    if (!content.endsWith("\n")) merged += "\n";

    merged += `===== FILE ${fileNameOnly} END =====\n\n`;
}

fs.writeFileSync(path.resolve(process.cwd(), outFile), merged, "utf8");
console.log(`Merged ${inputFiles.length} file(s) into ${outFile}`);
