import { render, screen } from "@testing-library/react";
import { Button } from "./Button";

it("renders Button with correct text", () => {
  render(
    <Button href="#" intent="primary" size="lg">
      Button
    </Button>,
  );
  expect(screen.getByRole("link")).toHaveTextContent("Button");
});

it("applies correct classes for primary intent and large size", () => {
  render(
    <Button href="#" intent="primary" size="lg">
      Button
    </Button>,
  );
  const button = screen.getByRole("link");
  expect(button).toHaveClass(
    "bg-blue-400 text-white hover:enabled:bg-blue-700 min-w-32 min-h-12 text-lg py-2.5 px-6",
  );
});
