import {
  getUserFromEnv,
  grabVirajmSite,
  grabVirajmSiteAsync,
  getTimeAndNfl,
  solveQuadratic,
} from "../index.js";

console.log("From native", getUserFromEnv());
console.log("virajm.com: ", grabVirajmSite());
const body = await grabVirajmSiteAsync();
console.log("virajm.com async: ", body);

const timeAndNfl = getTimeAndNfl();
console.log("timeAndNfl: ", timeAndNfl);

const coefficients = { a: 2, b: 6, c: 1 };
const solution = solveQuadratic(coefficients);
console.log("solution: ", solution);

try {
  // This should give no real roots and throw an error
  const coefficients = { a: 2, b: 1, c: 1 };
  const solution = solveQuadratic(coefficients);
  console.log("solution: ", solution);
} catch (error) {
  console.log("Caught error");
  console.log(error);
}
