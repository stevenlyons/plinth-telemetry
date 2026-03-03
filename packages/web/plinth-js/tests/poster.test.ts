import { beforeEach, describe, expect, it, mock } from "bun:test";
import { postBeacons } from "../src/poster.js";

// ── Tests ─────────────────────────────────────────────────────────────────────

describe("postBeacons", () => {
  const ENDPOINT = "http://localhost:3000/beacon";
  const PROJECT_KEY = "p123456789";
  const BATCH_JSON = JSON.stringify({ beacons: [{ seq: 0, event: "session_open" }] });

  beforeEach(() => {
    globalThis.fetch = mock(() => Promise.resolve(new Response(null, { status: 200 })));
  });

  it("sets method POST", async () => {
    await postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON);

    const [, init] = (globalThis.fetch as ReturnType<typeof mock>).mock.calls[0] as [string, RequestInit];
    expect(init.method).toBe("POST");
  });

  it("sets keepalive true", async () => {
    await postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON);

    const [, init] = (globalThis.fetch as ReturnType<typeof mock>).mock.calls[0] as [string, RequestInit];
    expect(init.keepalive).toBe(true);
  });

  it("sets Content-Type header to application/json", async () => {
    await postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON);

    const [, init] = (globalThis.fetch as ReturnType<typeof mock>).mock.calls[0] as [string, RequestInit];
    const headers = init.headers as Record<string, string>;
    expect(headers["Content-Type"]).toBe("application/json");
  });

  it("sets X-Project-Key header to the provided project key", async () => {
    await postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON);

    const [, init] = (globalThis.fetch as ReturnType<typeof mock>).mock.calls[0] as [string, RequestInit];
    const headers = init.headers as Record<string, string>;
    expect(headers["X-Project-Key"]).toBe(PROJECT_KEY);
  });

  it("sends batch JSON as body", async () => {
    await postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON);

    const [, init] = (globalThis.fetch as ReturnType<typeof mock>).mock.calls[0] as [string, RequestInit];
    expect(init.body).toBe(BATCH_JSON);
  });

  it("rejects when fetch throws a network error", async () => {
    globalThis.fetch = mock(() => Promise.reject(new TypeError("network error")));

    await expect(postBeacons(ENDPOINT, PROJECT_KEY, BATCH_JSON)).rejects.toThrow("network error");
  });
});
