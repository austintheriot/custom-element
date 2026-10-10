export function captureErrors(action) {
  const errors = [];
  const observe = event => {
    errors.push(event.error ?? new Error(event.message));
    event.preventDefault();
  };
  window.addEventListener('error', observe);
  try {
    action();
    return errors;
  } finally {
    window.removeEventListener('error', observe);
  }
}

export function nextTurn() {
  return new Promise(resolve => setTimeout(resolve, 0));
}

export function dispatchProbe(element, cancelable) {
  const event = new Event('probe', { bubbles: true, cancelable });
  const accepted = element.dispatchEvent(event);
  return { event, accepted, currentTarget: event.currentTarget,
    phase: event.eventPhase, path: event.composedPath() };
}

export function prepareProbe(element) {
  const ancestor = document.createElement('div');
  const counts = { target: 0, ancestor: 0 };
  element.addEventListener('probe', element);
  element.addEventListener('probe', () => counts.target++);
  ancestor.addEventListener('probe', () => counts.ancestor++);
  ancestor.append(element);
  return { ancestor, counts };
}

export function shadowProbe(host) {
  const root = host.attachShadow({ mode: 'open' });
  const button = document.createElement('button');
  root.append(button);
  host.addEventListener('probe', host);
  const event = new Event('probe', { bubbles: true, composed: true });
  button.dispatchEvent(event);
  return { button, event };
}

export function moveCapability() {
  return typeof Element.prototype.moveBefore === 'function';
}

export function moveBetweenParents(element) {
  const source = document.createElement('div');
  const destination = document.createElement('div');
  document.body.append(source, destination);
  source.append(element);
  destination.moveBefore(element, null);
  return { source, destination };
}

export function injectBridgeFailure(element) {
  const bridge = element.struct;
  const original = bridge.connectedCallback;
  let attempts = 0;
  let reported = 0;
  try {
    bridge.connectedCallback = () => {
      attempts++;
      throw new Error('controlled bridge failure');
    };
    try {
      element.connectedCallback();
    } catch (error) {
      if (error.message === 'controlled bridge failure') reported++;
    }
    return { attempts, reported };
  } finally {
    bridge.connectedCallback = original;
  }
}

export function plainUpgradeRecursion(name) {
  const element = document.createElement(name);
  document.body.append(element);
  let bodyCount = 0;
  let nested = null;
  class RecursiveElement extends HTMLElement {
    constructor() {
      super();
      bodyCount++;
      if (bodyCount === 1) nested = document.createElement(name);
    }
  }
  const errors = captureErrors(() => customElements.define(name, RecursiveElement));
  element.remove();
  return { bodyCount, errors, failedNested: nested instanceof HTMLUnknownElement };
}

export function isFailedElement(element) {
  return element instanceof HTMLUnknownElement;
}

export function connectWithPostEffectFailure(element) {
  const bridge = element.struct;
  const original = bridge.connectedCallback;
  try {
    bridge.connectedCallback = function () {
      original.call(this);
      throw new Error('controlled post-effect failure');
    };
    document.body.append(element);
  } finally {
    bridge.connectedCallback = original;
    element.remove();
  }
}
