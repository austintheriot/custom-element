const bridge = Symbol.for("custom-element.bridge");

export function createCustomElement(
  makeStruct,
  attributes,
  elementConstructor = HTMLElement,
) {
  class GeneratedCustomElement extends elementConstructor {
    [bridge];
    constructor(...args) {
      super();
      this[bridge] = makeStruct(this, args);
    }

    static get observedAttributes() {
      return attributes;
    }

    connectedCallback() {
      this[bridge].connectedCallback();
    }

    disconnectedCallback() {
      this[bridge].disconnectedCallback();
    }

    adoptedCallback() {
      this[bridge].adoptedCallback();
    }

    connectedMoveCallback() {
      this[bridge].connectedMoveCallback();
    }

    attributeChangedCallback(...args) {
      this[bridge].attributeChangedCallback(...args);
    }

    handleEvent(event) {
      this[bridge].handleEvent(event);
    }
  }

  return GeneratedCustomElement;
}
