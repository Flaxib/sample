process.stdin.resume();
process.stdin.setEncoding("utf8");

// console.log(process.argv);

function usage() {
  const scriptName = process.argv[1].split("/").at(-1);
  // console.log(scriptName);
  console.log(
    `Usage: docker network inspect bridge | node ${scriptName} <postgresqlContainerName>\n`
  );
  console.log("Example: ");
  console.log(
    `  docker network inspect bridge | node ${scriptName} diesel-demo\n\n`
  );

  process.exit(1);
}

if (process.argv.length < 3) {
  console.error("\nError: Except the 3rd argument <postgresqlContainerName>\n");
  usage();
}

const containerName = process.argv[2];

process.stdin.on("data", function (chunk) {
  let data = JSON.parse(chunk);
  const filteredContainer = Object.values(data[0].Containers).filter(
    (o) => o.Name === containerName
  );
  if (filteredContainer.length === 0) {
    console.log(`\nError: Container name "${containerName}" not found\n`);
    console.log(
      `Container names: \n  ${Object.values(data[0].Containers)
        .map((o) => o.Name)
        .join("\n  ")}\n`
    );
    usage();
  }
  const container = filteredContainer[0];
  console.log(container.IPv4Address.split("/")[0]);
});
