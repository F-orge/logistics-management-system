import {
	lazy,
	Match,
	onMount,
	Suspense,
	Switch,
	type Component,
} from "solid-js";
import { Navigate, Route, Router } from "@solidjs/router";
import {
	ColorModeProvider,
	ColorModeScript,
	createLocalStorageManager,
} from "@kobalte/core";
import { Toaster } from "./components/ui/toast.tsx";
import StatusPage from "./status.tsx";
import { Observer } from "tailwindcss-intersect";
import MarketingRoutes from "./routes/marketing/index.tsx";
import ManagementRoutes from "./routes/management/index.tsx";

/*-- Global Routes --*/
const NotFoundPage = lazy(() => import("./not-found.tsx"));

/*-- Employee Management --*/

const App: Component<{}> = (_props) => {
	const storageManager = createLocalStorageManager("rsbuild-ui-theme");

	onMount(() => {
		Observer.start();
	});

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
					<MarketingRoutes />
				</Match>
				<Match
					when={
						window.location.hostname ===
						(import.meta.env.PUBLIC_MANAGEMENT_HOST_NAME as string)
					}
				>
					<ManagementRoutes />
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
