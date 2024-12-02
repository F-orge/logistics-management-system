import { type Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import NotFoundPage from "./routes/not-found.tsx";
import HomePage from "./routes/index.tsx";
import ClientLoginPage from "./routes/client/login.tsx";
import AdminLoginPage from "./routes/admin/login.tsx";
import ClientRegisterPage from "./routes/client/register.tsx";

// deno-lint-ignore ban-types
const App: Component<{}> = (_props) => {
  return (
    <Router>
      <Route path="" component={HomePage} />
      <Route path="/login" component={ClientLoginPage} />
      <Route path="/register" component={ClientRegisterPage} />
      <Route path="/admin/login" component={AdminLoginPage} />
      <Route path={"*"} component={NotFoundPage} />
    </Router>
  );
};

export default App;
