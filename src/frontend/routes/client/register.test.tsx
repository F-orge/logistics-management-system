import RegisterPage from "./register.tsx";
import { expect, test } from "vitest";
import { render } from "@solidjs/testing-library";

test("renders client login page", () => {
  const { getByText } = render(() => <RegisterPage />);
  expect(getByText("Client registration page")).not.toBe(null);
});
