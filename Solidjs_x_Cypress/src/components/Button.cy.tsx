import Button from "./Button";

it("uses custom text for the button label", () => {
  cy.mount(() => <Button label="TEST" />);
  cy.get("button").should("contains.text", "TEST");
});
