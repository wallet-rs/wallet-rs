import { registerRootComponent } from "expo";
import { ExpoRoot } from "expo-router";

// From: https://expo.github.io/router/docs/troubleshooting/
// Must be exported or Fast Refresh won't update the context
export function App() {
  const ctx = require.context("./app");
  return <ExpoRoot context={ctx} />;
}

registerRootComponent(App);
