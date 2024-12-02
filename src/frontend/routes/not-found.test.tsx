import NotFoundPage from "./not-found.tsx";
import { expect,test } from "vitest";
import { render } from "@solidjs/testing-library";

test("renders not found page", () => {
  const { getByText } = render(() => <NotFoundPage />);
  expect(getByText("404 - Not Found")).not.toBe(null);
  console.log("Not found page rendered");
});
