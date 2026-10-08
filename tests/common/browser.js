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
