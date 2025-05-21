import {
  getUserFromEnv,
  grabVirajmSite,
  grabVirajmSiteAsync,
  getTimeAndNfl,
  solveQuadratic,
  sleepy,
  Client,
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

// This next section tests that async rust is nonblocking in Node
const sleepyPromise = sleepy();
const client = new Client("USER");
const value = client.getValue();
console.log(value);
const t0SiteText = await client.getTensorzeroWebsite();

let count = 0;
// Every 100ms increment the count
const intervalId = setInterval(() => {
  count++;
  // Logging the count to observe the effect, consistent with other logging in the file.
  console.log(`Current count: ${count}`);
}, 100);

// Attach handlers to sleepyPromise to perform actions when it resolves or rejects.
// This includes logging its outcome and clearing the interval so the program can exit.
sleepyPromise
  .then((message) => {
    console.log(message); // Log the success message from sleepy()
    clearInterval(intervalId); // Stop the counter
    console.log("Counter stopped after sleepy resolved.");
  })
  .catch((error) => {
    console.error("Error from sleepy():", error); // Log any error from sleepy()
    clearInterval(intervalId); // Stop the counter
    console.log("Counter stopped due to an error in sleepy.");
  });
// Block until sleepyPromise resolves
await sleepyPromise;

console.log(count);

console.log("tensorzero site");
console.log(t0SiteText);
