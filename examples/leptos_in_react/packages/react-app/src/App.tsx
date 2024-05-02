import { useCallback, useEffect, useRef, useState } from "react";
import initWasm, { getAppTagName, initApp } from "./compiled/leptos_app";

const wasmIsInit = initWasm();

function App() {
  const [showApp, setShowApp] = useState(false);
  // let wasm decide what the custom element's tag name should be
  const [AppTagName, setAppTagName] = useState<string | null>(null);
  // don't let React try to run `useEffect` more than once
  const isInitializing = useRef(false);

  const initializeApp = useCallback<() => void>(async () => {
    if (isInitializing.current) return;
    isInitializing.current = true;

    await wasmIsInit;

    initApp();
    setAppTagName(getAppTagName());
  }, []);

  useEffect(() => void initializeApp(), [initializeApp]);

  return (
    <>
      {showApp ? AppTagName && <AppTagName /> : null}

      <button onClick={() => setShowApp((prev) => !prev)}>
        {showApp ? "Hide app from React" : "Show app from React"}
      </button>
    </>
  );
}

export default App;
