import NotFoundPage from "./not-found.tsx";
import { expect, test } from "vitest";
import { render } from "@solidjs/testing-library";
import userEvent from "@testing-library/user-event";

const user = userEvent.setup();

test("renders not found page", () => {
  const { getByText } = render(() => <NotFoundPage />);
  expect(getByText("404 - Not Found")).not.toBe(null);
});
