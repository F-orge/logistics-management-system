import { lazy, Match, Suspense, Switch, type Component } from "solid-js";
import { Navigate, Route, Router } from "@solidjs/router";
import {
	ColorModeProvider,
	ColorModeScript,
	createLocalStorageManager,
} from "@kobalte/core";
import { Toaster } from "./components/ui/toast.tsx";
import StatusPage from "./status.tsx";

/*-- Global Routes --*/
const NotFoundPage = lazy(() => import("./not-found.tsx"));

/*-- Marketing Website --*/
const MarketingHomePage = lazy(
	() => import("./marketing-management/index.tsx"),
);
const MarketingAboutPage = lazy(
	() => import("./marketing-management/about.tsx"),
);
const MarketingServicesPage = lazy(
	() => import("./marketing-management/services.tsx"),
);

/*-- Employee Management --*/
const EmployeeManagementLoginPage = lazy(
	() => import("./employee-management/login.tsx"),
);

const EmployeeManagementDashboardPage = lazy(
	() => import("./employee-management/dashboard/index.tsx"),
);

const EmployeeManagementDashboardLayout = lazy(
	() => import("./employee-management/dashboard/layout.tsx"),
);

const App: Component<{}> = (_props) => {
	const storageManager = createLocalStorageManager("rsbuild-ui-theme");

	return (
		<Router
			root={(props) => (
				<>
					<ColorModeScript storageType={storageManager.type} />
					<ColorModeProvider storageManager={storageManager}>
						<Suspense>{props.children}</Suspense>
					</ColorModeProvider>
					<Toaster />
				</>
			)}
		>
			<Switch>
				<Match
					when={
						window.location.hostname ===
						(import.meta.env.PUBLIC_MARKETING_HOST_NAME as string)
					}
				>
					<Route path="/" component={MarketingHomePage} />
					<Route path={"/about"} component={MarketingAboutPage} />
					<Route path={"/services"} component={MarketingServicesPage} />
				</Match>
				<Match
					when={
						window.location.hostname ===
						(import.meta.env.PUBLIC_EMPLOYEE_HOST_NAME as string)
					}
				>
					<Route path="/" component={EmployeeManagementDashboardLayout}>
						<Route path={"/"} component={EmployeeManagementDashboardPage} />
					</Route>
					<Route path={"/login"} component={EmployeeManagementLoginPage} />
				</Match>
				<Match
					when={
						window.location.hostname ===
						(import.meta.env.PUBLIC_STATUS_HOST_NAME as string)
					}
				>
					<Route path={"/"} component={StatusPage} />
				</Match>
			</Switch>
			<Route path={"*"} component={() => <Navigate href={"not-found"} />} />
			<Route path={"/not-found"} component={NotFoundPage} />
		</Router>
	);
};

export default App;
