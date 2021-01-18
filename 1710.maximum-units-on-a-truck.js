/**
 * @param {number[][]} boxTypes
 * @param {number} truckSize
 * @return {number}
 */
var maximumUnits = function (boxTypes, truckSize) {
  boxTypes.sort((a, b) => {
    if (a[1] > b[1]) return -1;
    if (a[1] < b[1]) return 1;
    return 0;
  });

  let numOfUnits = 0;
  let numOfBoxes = 0;
  for (let i = 0; i < boxTypes.length; i++) {
    console.log('---', boxTypes[i]);
    if (numOfBoxes + boxTypes[i][0] < truckSize) {
      numOfUnits += boxTypes[i][0] * boxTypes[i][1];
      numOfBoxes += boxTypes[i][0];
    } else {
      numOfUnits += (truckSize - numOfBoxes) * boxTypes[i][1];
      return numOfUnits;
    }
    console.log('units: ', numOfUnits);
    console.log('box: ', numOfBoxes);
  }
  return numOfUnits;
};

console.log(
  maximumUnits(
    [
      [1, 3],
      [2, 2],
      [3, 1],
    ],
    4
  )
);

console.log(
  maximumUnits(
    [
      [5, 10],
      [2, 5],
      [4, 7],
      [3, 9],
    ],
    10
  )
);
