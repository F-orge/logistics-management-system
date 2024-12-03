import { type Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import NotFoundPage from "./routes/not-found.tsx";
import HomePage from "./routes/index.tsx";
import ClientLoginPage from "./routes/client/login.tsx";
import AdminLoginPage from "./routes/admin/login.tsx";
import ClientRegisterPage from "./routes/client/register.tsx";
import AdminDashboardLayout from "./routes/admin.tsx";
import ShipmentCargoPage from "./routes/admin/shipments/cargo.tsx";
import ShipmentTransportPage from "./routes/admin/shipments/transport.tsx";
import ShipmentPage from "./routes/admin/shipments/index.tsx";
import WarehousePage from "./routes/admin/warehouse/index.tsx";
import ProfilePage from "./routes/admin/management/profile.tsx";
import AssignmentPage from "./routes/admin/management/assignment.tsx";

const App: Component<{}> = (_props) => {
  return (
    <Router>
      <Route path="" component={HomePage} />
      <Route path="/login" component={ClientLoginPage} />
      <Route path="/register" component={ClientRegisterPage} />
      <Route path="/admin/login" component={AdminLoginPage} />
      <Route path={"/admin/"} component={AdminDashboardLayout}>
        <Route path="/profile" component={ProfilePage} />
        <Route path="/warehouse" component={WarehousePage} />
        <Route path="/management" component={AdminLoginPage} />
        <Route path="/management/assignment" component={AssignmentPage} />
        <Route path="/shipment" component={ShipmentPage} />
        <Route path="/shipment/transport" component={ShipmentTransportPage} />
        <Route path="/shipment/cargo/:id" component={ShipmentCargoPage} />
      </Route>
      <Route path={"*"} component={NotFoundPage} />
    </Router>
  );
};

export default App;
