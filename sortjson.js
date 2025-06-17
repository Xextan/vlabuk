let input = "";
process.stdin.on("data", data => {input += data;});
process.stdin.on("end", () => {
  const data = JSON.parse(input);
  if (!Array.isArray(data)) {
    console.error("uh oh the json isn't a list");
    process.exit(1);
  }
  const sorted = data.sort((a, b) => a.word.localeCompare(b.word));
  console.log(JSON.stringify(sorted, null, 2));
});
