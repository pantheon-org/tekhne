#!/usr/bin/env node
// Fetches the Context Radar tool-comparison dataset and prints it as JSON on stdout.
//
// The dataset ({meta, layers, tools}) is published as a plain JSON file in the public
// Context Radar repository, so it is fetched and parsed as data. Nothing fetched is
// ever executed. Pass a different URL as the first argument to read a fork or a
// pinned commit instead of main.

const defaultUrl =
  "https://raw.githubusercontent.com/thoroc/context-radar/main/data/context-reduction-tools.json";
const url = process.argv[2] || defaultUrl;

const isDataset = (data) =>
  data !== null &&
  typeof data === "object" &&
  typeof data.meta === "object" &&
  Array.isArray(data.layers) &&
  Array.isArray(data.tools);

const main = async () => {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Fetch failed ${res.status}: ${url}`);
  const data = await res.json();
  if (!isDataset(data)) {
    throw new Error(
      `Unexpected dataset shape at ${url}: expected an object with meta, layers and tools.`,
    );
  }
  process.stdout.write(JSON.stringify(data));
};

main().catch((err) => {
  console.error(err.message);
  process.exit(1);
});
