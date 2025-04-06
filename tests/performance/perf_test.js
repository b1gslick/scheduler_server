import http from "k6/http";
import { check } from "k6";
import exec from "k6/execution";
import { createRandomString } from "./utils.js";

// test configuration
export const options = {
  scenarios: {
    simple_perf_scenario: {
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
        delayAbortEval: "10s",
      },
    ],
    http_req_duration: [
      {
        threshold: "p(99) < 50", // below < 20ms
        abortOnFail: true,
        delayAbortEval: "10s",
      },
    ],
    http_req_failed: [
      {
        threshold: "rate<0.05", // http error less than 1%
        abortOnFail: true,
        delayAbortEval: "10s",
      },
    ],
  },
};

const baseUrl = `${__ENV.BASE_URL}`;

export function setup() {
  const userParams = {
    email: `${createRandomString(10)}@test.iv`,
    password: "SomestrongPassword1$@",
  };

  const reg = http.post(`${baseUrl}/registration`, JSON.stringify(userParams));

  if (reg.status !== 200) {
    console.log(reg);
  }

  const login = http.post(`${baseUrl}/login`, JSON.stringify(userParams));
  const token = JSON.parse(login.body).token;
  return token;
}

export default function (token) {
  const params = {
    headers: {
      "Content-Type": "application/json",
      Authorization: token,
    },
  };

  const body = {
    title: `${exec.scenario.name}`,
    content: `${exec.scenario.startTime}`,
    time: parseInt(`${exec.vu.iterationInInstance}`),
  };

  let add = http.post(`${baseUrl}/activity`, JSON.stringify(body), params);
  check(add, { "status was 201": (r) => r.status === 201 });
  if (add.status !== 201) {
    console.log(add);
  }

  const time_body = {
    time: parseInt(`${exec.vu.iterationInInstance}`),
    activity_id: parseInt(`${exec.vu.iterationInInstance}`) + 1,
  };

  let add_time = http.post(
    `${baseUrl}/time_spent`,
    JSON.stringify(time_body),
    params,
  );
  check(add_time, { "status was 201": (r) => r.status === 201 });
  if (add_time.status !== 201) {
    console.log(add_time);
  }

  const id = parseInt(`${exec.vu.iterationInInstance}`) + 1;
  const update_body = {
    id: id,
    title: `updated`,
    content: `${exec.scenario.startTime}`,
    time: parseInt(`${exec.vu.iterationInInstance}`),
  };

  let update = http.put(
    `${baseUrl}/activity/${id}`,
    JSON.stringify(update_body),
    params,
  );
  if (update.status !== 201) {
    console.log(update);
  }
  check(update, { "status was 201": (r) => r.status === 201 });
}
