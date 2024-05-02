export function createCustomElement(
  makeStruct,
  attributes,
  elementConstructor = HTMLElement,
) {
  class GeneratedCustomElement extends elementConstructor {
    struct;
    constructor(...args) {
      super();
      this.struct = makeStruct(this, args);
    }

    static get observedAttributes() {
      return attributes;
    }

    connectedCallback() {
      this.struct.connectedCallback();
    }

    disconnectedCallback() {
      this.struct.connectedCallback();
    }

    adoptedCallback() {
      this.struct.adoptedCallback();
    }

    attributeChangedCallback(...args) {
      this.struct.attributeChangedCallback(...args);
    }

    handleEvent(event) {
      this.struct.handleEvent(event);
    }
  }

  return GeneratedCustomElement;
}
