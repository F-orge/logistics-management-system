import LoginPage from "./login.tsx";
import { expect, test } from "vitest";
import { render } from "@solidjs/testing-library";

test("renders client login page", () => {
  const { getByText } = render(() => <LoginPage />);
  expect(getByText("Client login page")).not.toBe(null);
});
