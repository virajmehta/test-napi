import {
  getUserFromEnv,
  grabVirajmSite,
  grabVirajmSiteAsync,
  getTimeAndNfl,
} from "../index.js";

console.log("From native", getUserFromEnv());
console.log("virajm.com: ", grabVirajmSite());
const body = await grabVirajmSiteAsync();
console.log("virajm.com async: ", body);

const timeAndNfl = getTimeAndNfl();
console.log("timeAndNfl: ", timeAndNfl);
