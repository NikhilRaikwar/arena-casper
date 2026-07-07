import { createHash, randomBytes } from "node:crypto";

export function sha256(input: string): string {
  return createHash("sha256").update(input).digest("hex");
}

export function deployHash(prefix = "mock-deploy"): string {
  return `${prefix}-${randomBytes(16).toString("hex")}`;
}
