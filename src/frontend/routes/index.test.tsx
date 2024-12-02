import HomePage from "./index.tsx";
import { expect, test } from "vitest";
import { render } from "@solidjs/testing-library";

test("renders not found page", () => {
  const { getByText } = render(() => <HomePage />);
  expect(getByText("Home page")).not.toBe(null);
});
