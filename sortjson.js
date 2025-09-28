let input = "";
process.stdin.on("data", data => { input += data; });
process.stdin.on("end", () => {
  const wrapper = JSON.parse(input);
  if (!wrapper.data || !Array.isArray(wrapper.data)) {
    console.error("uh oh the json's .data isn't a list");
    process.exit(1);
  }
  const sorted = wrapper.data.sort((a, b) => a.word.localeCompare(b.word));
  const result = { "$schema": wrapper.schema || "./words.schema.json", "data": sorted };
  console.log(JSON.stringify(result, null, 2));
});
