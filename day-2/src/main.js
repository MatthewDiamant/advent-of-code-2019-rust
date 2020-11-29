let fs = require("fs");

fs.readFile("./input.txt", "utf8", function(err, data) {
  let input = data.split(",").map(n => parseInt(n));
  input[1] = 12;
  input[2] = 2;
  let current_position = 0;
  while (input[current_position] !== 99) {
    let input_1 = current_position + 1;
    let input_2 = current_position + 2;
    let output = current_position + 3;
    let location_1 = input[input_1];
    let location_2 = input[input_2];
    let output_location = input[output];
    if (input[current_position] === 1) {
      input[output_location] = input[location_1] + input[location_2];
    }
    if (input[current_position] === 2) {
      input[output_location] = input[location_1] * input[location_2];
    }
    current_position += 4;
  }
  console.log(input[0]);
});
