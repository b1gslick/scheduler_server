import http from "k6/http";
import { check } from "k6";
import exec from "k6/execution";

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

  const time_body = {
    time: parseInt(`${exec.vu.iterationInInstance}`),
    activity_id: parseInt(`${exec.vu.iterationInInstance}`) + 1,
  };

  let add_time = http.post(
    `${baseUrl}/time_spent`,
    JSON.stringify(time_body),
    params,
  );
  check(add_time, { "status was 200": (r) => r.status === 200 });
  if (add_time.status !== 200) {
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
    `${baseUrl}/activities/${id}`,
    JSON.stringify(update_body),
    params,
  );
  if (update.status !== 200) {
    console.log(update);
  }
  check(update, { "status was 200": (r) => r.status === 200 });
}
