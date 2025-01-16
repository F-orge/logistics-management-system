import { Route } from "@solidjs/router";
import { lazy, type Component } from "solid-js";
import DashboardLayout from "~/layouts/dashboard.tsx";

const EmployeeManagementLoginPage = lazy(() => import("./login.tsx"));
const ManagementOverviewPage = lazy(() => import("./dashboard/index.tsx"));

const ManagementRoutes: Component<{}> = (props) => {
	return (
		<>
			<Route path={"/login"} component={EmployeeManagementLoginPage} />
			<Route path={"/"} component={DashboardLayout}>
				<Route path={"/"} component={ManagementOverviewPage} />
			</Route>
		</>
	);
};

export default ManagementRoutes;
