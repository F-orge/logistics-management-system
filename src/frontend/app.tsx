import { lazy, Match, Suspense, Switch, type Component } from "solid-js";
import { Navigate, Route, Router } from "@solidjs/router";
import {
	ColorModeProvider,
	ColorModeScript,
	createLocalStorageManager,
} from "@kobalte/core";

import NotFoundPage from "./not-found.tsx";
import { Toaster } from "~/components/ui/toast.tsx";

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
				<Match when={window.location.hostname === "www.localhost"}>
					<Route path="/" component={MarketingHomePage} />
					<Route path={"/about"} component={MarketingAboutPage} />
					<Route path={"/services"} component={MarketingServicesPage} />
				</Match>
			</Switch>
			<Route path={"*"} component={() => <Navigate href={"not-found"} />} />
			<Route path={"/not-found"} component={NotFoundPage} />
		</Router>
	);
};

export default App;
