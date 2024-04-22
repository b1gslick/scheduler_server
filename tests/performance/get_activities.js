import http from "k6/http";
import { check } from "k6";
import exec from "k6/execution";

// test configuration
export const options = {
  scenarios: {
    activities: {
      executor: "constant-arrival-rate",
      duration: "2m",
      rate: 30,
      timeUnit: "1m",
      preAllocatedVUs: 1,
    },
  },
  thresholds: {
    checks: [
      {
        threshold: "rate>0.99", // check should be pass more then 99%
        abortOnFail: true,
        delayAbortEval: "5s",
      },
    ],
    http_req_duration: [
      {
        threshold: "p(99) < 300", // below < 300ms
        abortOnFail: true,
        delayAbortEval: "5s",
      },
    ],
    http_req_failed: [
      {
        threshold: "rate<0.01", // http error less than 1%
        abortOnFail: true,
        delayAbortEval: "5s",
      },
    ],
  },
};

const baseUrl = `${__ENV.BASE_URL}`;

export function setup() {
  const userParams = {
    email: "perf@test.iv",
    password: "somestrongPassword1",
  };

  const reg = http.post(`${baseUrl}/registration`, JSON.stringify(userParams));

  if (reg.status !== 200) {
    console.log(reg);
  }

  const login = http.post(`${baseUrl}/login`, JSON.stringify(userParams));
  const token = login.body.replaceAll(`"`, "");
  return token;
}

export default function (token) {
  const params = {
    headers: {
      "Content-Type": "application/json",
      Authorization: token,
    },
  };
  let get = http.get(`${baseUrl}/activities?limit=100000&offset=0`, params);

  check(get, { "status was 200": (r) => r.status === 200 });
  if (get.status !== 200) {
    console.log(get);
  }

  const body = {
    title: `${exec.scenario.name}`,
    content: `${exec.scenario.startTime}`,
    time: parseInt(`${exec.vu.iterationInInstance}`),
  };

  let add = http.post(`${baseUrl}/activities`, JSON.stringify(body), params);
  check(add, { "status was 200": (r) => r.status === 200 });
  if (add.status !== 200) {
    console.log(add);
  }

  let delete_activity = http.del(
    `${baseUrl}/activities/${parseInt(`${exec.vu.iterationInInstance}`) + 1}`,
    {},
    params,
  );
  check(delete_activity, { "status was 200": (r) => r.status === 200 });
  if (delete_activity.status !== 200) {
    console.log(delete_activity);
  }
}
