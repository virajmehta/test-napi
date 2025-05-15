import {
  getUserFromEnv,
  grabVirajmSite,
  grabVirajmSiteAsync,
} from "../index.js";

console.log("From native", getUserFromEnv());
console.log("virajm.com: ", grabVirajmSite());
const body = await grabVirajmSiteAsync();
console.log("virajm.com async: ", body);
