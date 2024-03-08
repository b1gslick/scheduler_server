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
        threshold: "p(99) < 20", // below < 20ms
        abortOnFail: true,
        delayAbortEval: "10s",
      },
    ],
    http_req_failed: [
      {
        threshold: "rate<0.01", // http error less than 1%
        abortOnFail: true,
        delayAbortEval: "10s",
      },
    ],
  },
};

// Simulate user behavior
export default function () {
  let baseUrl = "http://backend:8080";

  const body = {
    id: `${exec.vu.iterationInInstance}`,
    title: `${exec.scenario.name}`,
    content: `${exec.scenario.startTime}`,
    time: parseInt(`${exec.vu.iterationInInstance}`),
  };

  let add = http.post(`${baseUrl}/activities`, JSON.stringify(body));
  check(add, { "status was 200": (r) => r.status === 200 });

  const time_body = {
    time: parseInt(`${exec.vu.iterationInInstance}`),
    activity_id: `${exec.vu.iterationInInstance}`,
  };

  let add_time = http.post(`${baseUrl}/time_spent`, JSON.stringify(time_body));
  check(add_time, { "status was 200": (r) => r.status === 200 });

  const update_body = {
    id: `${exec.vu.iterationInInstance}`,
    title: `updated`,
    content: `${exec.scenario.startTime}`,
    time: parseInt(`${exec.vu.iterationInInstance}`),
  };

  let update = http.put(
    `${baseUrl}/activities/${exec.vu.iterationInInstance}`,
    JSON.stringify(update_body),
  );
  check(update, { "status was 200": (r) => r.status === 200 });
}
