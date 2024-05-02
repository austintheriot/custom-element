import { useCallback, useEffect, useRef, useState } from "react";
import initWasm, { getAppTagName, initApp, State } from "./compiled/leptos_app";

const wasmIsInit = initWasm();

interface AppData {
  appTagName: string;
  appState: State;
}

function App() {
  const [showApp, setShowApp] = useState(false);
  // let wasm decide what the custom element's tag name should be
  const [appData, setAppData] = useState<AppData | null>(null);
  // don't let React try to run `useEffect` more than once
  const isInitializing = useRef(false);

  const initializeApp = useCallback<() => void>(async () => {
    if (isInitializing.current) return;
    isInitializing.current = true;

    await wasmIsInit;

    const state = initApp();
    setAppData({
      appTagName: getAppTagName(),
      appState: state,
    });
  }, []);

  useEffect(() => void initializeApp(), [initializeApp]);

  const App = appData?.appTagName;

  return (
    <>
      {showApp && App ? <App /> : null}

      <button onClick={() => appData?.appState.incrementCountBy(100)}>
        Increment count by 100 from React
      </button>
      <button onClick={() => setShowApp((prev) => !prev)}>
        {showApp ? "Hide app from React" : "Show app from React"}
      </button>
    </>
  );
}

export default App;
