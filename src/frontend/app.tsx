import { Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import NotFoundPage from "./routes/not-found.tsx";

// deno-lint-ignore ban-types
const App: Component<{}> = (_props) => {
  return (
    <Router>
      <Route path="" component={NotFoundPage}></Route>
      <Route path={"*"} component={NotFoundPage}></Route>
    </Router>
  );
};

export default App;
