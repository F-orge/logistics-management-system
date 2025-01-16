import { Route } from "@solidjs/router";
import { lazy, type Component } from "solid-js";

/*-- Marketing Website --*/
const MarketingHomePage = lazy(() => import("./landing.tsx"));

const MarketingRoutes: Component<{}> = (props) => {
	return (
		<>
			<Route path="/" component={MarketingHomePage} />
		</>
	);
};

export default MarketingRoutes;
