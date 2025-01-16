import { Route } from "@solidjs/router";
import { lazy, type Component } from "solid-js";

const EmployeeManagementLoginPage = lazy(() => import("./login.tsx"));

const ManagementRoutes: Component<{}> = (props) => {
	return (
		<>
			<Route path={"/login"} component={EmployeeManagementLoginPage} />
		</>
	);
};

export default ManagementRoutes;
